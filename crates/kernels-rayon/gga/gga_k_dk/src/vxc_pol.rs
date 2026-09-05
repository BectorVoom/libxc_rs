//! GGA_K_DK vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_dk.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_dk_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_aa_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    param_bb_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa_1 = f64x8::splat(param_aa_1);
    let param_aa_2 = f64x8::splat(param_aa_2);
    let param_aa_3 = f64x8::splat(param_aa_3);
    let param_aa_4 = f64x8::splat(param_aa_4);
    let param_aa_0 = f64x8::splat(param_aa_0);
    let param_bb_1 = f64x8::splat(param_bb_1);
    let param_bb_2 = f64x8::splat(param_bb_2);
    let param_bb_3 = f64x8::splat(param_bb_3);
    let param_bb_4 = f64x8::splat(param_bb_4);
    let param_bb_0 = f64x8::splat(param_bb_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = t6 * t28;
            let t30 = (simd::cbrt(t7));
            let t31 = t30 * t30;
            let t32 = param_aa_0;
            let t33 = param_aa_1;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = param_aa_2;
            let t42 = v_sigma0 * v_sigma0;
            let t43 = t41 * t42;
            let t44 = t35 * t35;
            let t45 = t44 * v_rho0;
            let t47 = f64x8::splat(1.0) / t36 / t45;
            let t49 = param_aa_3;
            let t50 = t42 * v_sigma0;
            let t51 = t49 * t50;
            let t52 = t44 * t44;
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = param_aa_4;
            let t56 = t42 * t42;
            let t57 = t55 * t56;
            let t58 = t52 * t35;
            let t60 = f64x8::splat(1.0) / t37 / t58;
            let t62 = t34 * t39 + t43 * t47 + t51 * t53 + t57 * t60 + t32;
            let t63 = t31 * t62;
            let t64 = param_bb_0;
            let t65 = param_bb_1;
            let t66 = t65 * v_sigma0;
            let t68 = param_bb_2;
            let t69 = t68 * t42;
            let t71 = param_bb_3;
            let t72 = t71 * t50;
            let t74 = param_bb_4;
            let t75 = t74 * t56;
            let t77 = t66 * t39 + t69 * t47 + t72 * t53 + t75 * t60 + t64;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t63 * t78;
            let t82 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t29 * t79));
            let t83 = (v_rho1).simd_le(dens_threshold);
            let t84 = -t17;
            let t86 = ((t15).select(t12, (t11).select(t16, t84 * t8)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t90 = t89 * t89;
            let t92 = ((t88).select(t24, t90 * t87));
            let t93 = t6 * t92;
            let t94 = t33 * v_sigma2;
            let t95 = v_rho1 * v_rho1;
            let t96 = (simd::cbrt(v_rho1));
            let t97 = t96 * t96;
            let t99 = f64x8::splat(1.0) / t97 / t95;
            let t101 = v_sigma2 * v_sigma2;
            let t102 = t41 * t101;
            let t103 = t95 * t95;
            let t104 = t103 * v_rho1;
            let t106 = f64x8::splat(1.0) / t96 / t104;
            let t108 = t101 * v_sigma2;
            let t109 = t49 * t108;
            let t110 = t103 * t103;
            let t111 = f64x8::splat(1.0) / t110;
            let t113 = t101 * t101;
            let t114 = t55 * t113;
            let t115 = t110 * t95;
            let t117 = f64x8::splat(1.0) / t97 / t115;
            let t119 = t102 * t106 + t109 * t111 + t114 * t117 + t94 * t99 + t32;
            let t120 = t31 * t119;
            let t121 = t65 * v_sigma2;
            let t123 = t68 * t101;
            let t125 = t71 * t108;
            let t127 = t74 * t113;
            let t129 = t123 * t106 + t125 * t111 + t127 * t117 + t121 * t99 + t64;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t120 * t130;
            let t134 = ((t83).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t93 * t131));
            let tzk0 = t82 + t134;
            acc_zk = tzk0;
            let t135 = t7 * t7;
            let t136 = f64x8::splat(1.0) / t135;
            let t137 = t17 * t136;
            let t139 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t137)));
            let t142 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t139));
            let t143 = t6 * t142;
            let t146 = f64x8::splat(1.0) / t30;
            let t147 = t146 * t62;
            let t148 = t147 * t78;
            let t150 = t29 * t148 / f64x8::splat(10.0);
            let t151 = t35 * v_rho0;
            let t153 = f64x8::splat(1.0) / t37 / t151;
            let t156 = t44 * t35;
            let t158 = f64x8::splat(1.0) / t36 / t156;
            let t161 = t52 * v_rho0;
            let t162 = f64x8::splat(1.0) / t161;
            let t165 = t52 * t151;
            let t167 = f64x8::splat(1.0) / t37 / t165;
            let t170 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t34 * t153 - f64x8::splat(16.0) / f64x8::splat(3.0) * t43 * t158 - f64x8::splat(8.0) * t51 * t162 - f64x8::splat(32.0) / f64x8::splat(3.0) * t57 * t167;
            let t171 = t31 * t170;
            let t172 = t171 * t78;
            let t175 = t77 * t77;
            let t176 = f64x8::splat(1.0) / t175;
            let t185 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t66 * t153 - f64x8::splat(16.0) / f64x8::splat(3.0) * t69 * t158 - f64x8::splat(8.0) * t72 * t162 - f64x8::splat(32.0) / f64x8::splat(3.0) * t75 * t167;
            let t186 = t176 * t185;
            let t187 = t63 * t186;
            let t191 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t143 * t79 + t150 + f64x8::splat(3.0) / f64x8::splat(20.0) * t29 * t172 - f64x8::splat(3.0) / f64x8::splat(20.0) * t29 * t187));
            let t192 = t84 * t136;
            let t194 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t192)));
            let t197 = ((t88).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t90 * t194));
            let t198 = t6 * t197;
            let t201 = t146 * t119;
            let t202 = t201 * t130;
            let t204 = t93 * t202 / f64x8::splat(10.0);
            let t206 = ((t83).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t198 * t131 + t204));
            let tvrho0 = t82 + t134 + t7 * (t191 + t206);
            acc_vrho_0 = tvrho0;
            let t210 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t137)));
            let t213 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t210));
            let t214 = t6 * t213;
            let t218 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t214 * t79 + t150));
            let t220 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t192)));
            let t223 = ((t88).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t90 * t220));
            let t224 = t6 * t223;
            let t227 = t95 * v_rho1;
            let t229 = f64x8::splat(1.0) / t97 / t227;
            let t232 = t103 * t95;
            let t234 = f64x8::splat(1.0) / t96 / t232;
            let t237 = t110 * v_rho1;
            let t238 = f64x8::splat(1.0) / t237;
            let t241 = t110 * t227;
            let t243 = f64x8::splat(1.0) / t97 / t241;
            let t246 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t94 * t229 - f64x8::splat(16.0) / f64x8::splat(3.0) * t102 * t234 - f64x8::splat(8.0) * t109 * t238 - f64x8::splat(32.0) / f64x8::splat(3.0) * t114 * t243;
            let t247 = t31 * t246;
            let t248 = t247 * t130;
            let t251 = t129 * t129;
            let t252 = f64x8::splat(1.0) / t251;
            let t261 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t121 * t229 - f64x8::splat(16.0) / f64x8::splat(3.0) * t123 * t234 - f64x8::splat(8.0) * t125 * t238 - f64x8::splat(32.0) / f64x8::splat(3.0) * t127 * t243;
            let t262 = t252 * t261;
            let t263 = t120 * t262;
            let t267 = ((t83).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t224 * t131 + t204 + f64x8::splat(3.0) / f64x8::splat(20.0) * t93 * t248 - f64x8::splat(3.0) / f64x8::splat(20.0) * t93 * t263));
            let tvrho1 = t82 + t134 + t7 * (t218 + t267);
            acc_vrho_1 = tvrho1;
            let t271 = t41 * v_sigma0;
            let t274 = t49 * t42;
            let t277 = t55 * t50;
            let t280 = f64x8::splat(2.0) * t271 * t47 + f64x8::splat(3.0) * t274 * t53 + f64x8::splat(4.0) * t277 * t60 + t33 * t39;
            let t281 = t31 * t280;
            let t282 = t281 * t78;
            let t285 = t68 * v_sigma0;
            let t288 = t71 * t42;
            let t291 = t74 * t50;
            let t294 = f64x8::splat(2.0) * t285 * t47 + f64x8::splat(3.0) * t288 * t53 + f64x8::splat(4.0) * t291 * t60 + t65 * t39;
            let t295 = t176 * t294;
            let t296 = t63 * t295;
            let t300 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t29 * t282 - f64x8::splat(3.0) / f64x8::splat(20.0) * t29 * t296));
            let tvsigma0 = t7 * t300;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t302 = t41 * v_sigma2;
            let t305 = t49 * t101;
            let t308 = t55 * t108;
            let t311 = f64x8::splat(2.0) * t302 * t106 + f64x8::splat(3.0) * t305 * t111 + f64x8::splat(4.0) * t308 * t117 + t33 * t99;
            let t312 = t31 * t311;
            let t313 = t312 * t130;
            let t316 = t68 * v_sigma2;
            let t319 = t71 * t101;
            let t322 = t74 * t108;
            let t325 = f64x8::splat(2.0) * t316 * t106 + f64x8::splat(3.0) * t319 * t111 + f64x8::splat(4.0) * t322 * t117 + t65 * t99;
            let t326 = t252 * t325;
            let t327 = t120 * t326;
            let t331 = ((t83).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t93 * t313 - f64x8::splat(3.0) / f64x8::splat(20.0) * t93 * t327));
            let tvsigma2 = t7 * t331;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
