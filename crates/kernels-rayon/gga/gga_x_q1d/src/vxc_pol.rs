//! GGA_X_Q1D vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
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
pub fn gga_x_q1d_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t40;
            let t44 = f64x8::splat(0.646416) / t42;
            let t46 = t28 * t28;
            let t48 = f64x8::splat(1.0) / t30 / t29;
            let t49 = t46 * t48;
            let t50 = v_sigma0 * v_sigma0;
            let t51 = t34 * t34;
            let t52 = t51 * v_rho0;
            let t54 = f64x8::splat(1.0) / t35 / t52;
            let t57 = t49 * t50 * t54 / f64x8::splat(576.0);
            let t58 = t40 / f64x8::splat(24.0) + t57;
            let t59 = t29 * t29;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t50 * v_sigma0;
            let t62 = t60 * t61;
            let t63 = t51 * t51;
            let t64 = f64x8::splat(1.0) / t63;
            let t67 = f64x8::splat(1.0) + t57 + t62 * t64 / f64x8::splat(2304.0);
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t58 * t68;
            let t71 = (f64x8::splat(1.804) - t44) * t28;
            let t72 = t32 * v_sigma0;
            let t76 = -t71 * t72 * t38 / f64x8::splat(24.0) + f64x8::splat(0.06525);
            let t78 = f64x8::splat(1.804) - t44 + t69 * t76;
            let t82 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t78));
            let t83 = (v_rho1).simd_le(dens_threshold);
            let t84 = -t16;
            let t86 = ((t14).select(t11, (t10).select(t15, t84 * t7)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t91 = ((t88).select(t22, t89 * t87));
            let t92 = t91 * t26;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t99 = t33 * v_sigma2 * t97;
            let t101 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t99;
            let t103 = f64x8::splat(0.646416) / t101;
            let t105 = v_sigma2 * v_sigma2;
            let t106 = t93 * t93;
            let t107 = t106 * v_rho1;
            let t109 = f64x8::splat(1.0) / t94 / t107;
            let t112 = t49 * t105 * t109 / f64x8::splat(576.0);
            let t113 = t99 / f64x8::splat(24.0) + t112;
            let t114 = t105 * v_sigma2;
            let t115 = t60 * t114;
            let t116 = t106 * t106;
            let t117 = f64x8::splat(1.0) / t116;
            let t120 = f64x8::splat(1.0) + t112 + t115 * t117 / f64x8::splat(2304.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = (f64x8::splat(1.804) - t103) * t28;
            let t125 = t32 * v_sigma2;
            let t129 = -t124 * t125 * t97 / f64x8::splat(24.0) + f64x8::splat(0.06525);
            let t131 = f64x8::splat(1.804) - t103 + t122 * t129;
            let t135 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t131));
            let tzk0 = t82 + t135;
            acc_zk = tzk0;
            let t136 = t6 * t6;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t16 * t137;
            let t140 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t138)));
            let t143 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t140));
            let t144 = t143 * t26;
            let t148 = t26 * t26;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t25 * t149;
            let t153 = t5 * t150 * t78 / f64x8::splat(8.0);
            let t154 = t42 * t42;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t155 * t28;
            let t157 = t34 * v_rho0;
            let t159 = f64x8::splat(1.0) / t36 / t157;
            let t160 = t72 * t159;
            let t166 = t51 * t34;
            let t168 = f64x8::splat(1.0) / t35 / t166;
            let t171 = t49 * t50 * t168 / f64x8::splat(108.0);
            let t172 = -t33 * v_sigma0 * t159 / f64x8::splat(9.0) - t171;
            let t173 = t172 * t68;
            let t175 = t67 * t67;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t58 * t176;
            let t178 = t63 * v_rho0;
            let t179 = f64x8::splat(1.0) / t178;
            let t182 = -t171 - t62 * t179 / f64x8::splat(288.0);
            let t183 = t76 * t182;
            let t185 = t155 * t46;
            let t186 = t48 * t50;
            let t192 = f64x8::splat(0.0003694650205761317) * t185 * t186 * t168 + t71 * t160 / f64x8::splat(9.0);
            let t194 = -f64x8::splat(0.00886716049382716) * t156 * t160 + t173 * t76 - t177 * t183 + t69 * t192;
            let t199 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t144 * t78 - t153 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t194));
            let t200 = t84 * t137;
            let t202 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t200)));
            let t205 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t202));
            let t206 = t205 * t26;
            let t210 = t91 * t149;
            let t213 = t5 * t210 * t131 / f64x8::splat(8.0);
            let t215 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t131 - t213));
            let tvrho0 = t82 + t135 + t6 * (t199 + t215);
            acc_vrho_0 = tvrho0;
            let t219 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t138)));
            let t222 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t219));
            let t223 = t222 * t26;
            let t228 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t78 - t153));
            let t230 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t200)));
            let t233 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t230));
            let t234 = t233 * t26;
            let t238 = t101 * t101;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t239 * t28;
            let t241 = t93 * v_rho1;
            let t243 = f64x8::splat(1.0) / t95 / t241;
            let t244 = t125 * t243;
            let t250 = t106 * t93;
            let t252 = f64x8::splat(1.0) / t94 / t250;
            let t255 = t49 * t105 * t252 / f64x8::splat(108.0);
            let t256 = -t33 * v_sigma2 * t243 / f64x8::splat(9.0) - t255;
            let t257 = t256 * t121;
            let t259 = t120 * t120;
            let t260 = f64x8::splat(1.0) / t259;
            let t261 = t113 * t260;
            let t262 = t116 * v_rho1;
            let t263 = f64x8::splat(1.0) / t262;
            let t266 = -t255 - t115 * t263 / f64x8::splat(288.0);
            let t267 = t129 * t266;
            let t269 = t239 * t46;
            let t270 = t48 * t105;
            let t276 = f64x8::splat(0.0003694650205761317) * t269 * t270 * t252 + t124 * t244 / f64x8::splat(9.0);
            let t278 = -f64x8::splat(0.00886716049382716) * t240 * t244 + t257 * t129 - t261 * t267 + t122 * t276;
            let t283 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t234 * t131 - t213 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t278));
            let tvrho1 = t82 + t135 + t6 * (t228 + t283);
            acc_vrho_1 = tvrho1;
            let t286 = t32 * t38;
            let t293 = t49 * v_sigma0 * t54 / f64x8::splat(288.0);
            let t294 = t33 * t38 / f64x8::splat(24.0) + t293;
            let t295 = t294 * t68;
            let t297 = t60 * t50;
            let t300 = t293 + t297 * t64 / f64x8::splat(768.0);
            let t301 = t76 * t300;
            let t303 = t48 * t54;
            let t309 = -f64x8::splat(0.00013854938271604938) * t185 * t303 * v_sigma0 - t71 * t286 / f64x8::splat(24.0);
            let t311 = f64x8::splat(0.0033251851851851854) * t156 * t286 + t295 * t76 - t177 * t301 + t69 * t309;
            let t315 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t311));
            let tvsigma0 = t6 * t315;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t316 = t32 * t97;
            let t323 = t49 * v_sigma2 * t109 / f64x8::splat(288.0);
            let t324 = t33 * t97 / f64x8::splat(24.0) + t323;
            let t325 = t324 * t121;
            let t327 = t60 * t105;
            let t330 = t323 + t327 * t117 / f64x8::splat(768.0);
            let t331 = t129 * t330;
            let t333 = t48 * t109;
            let t339 = -f64x8::splat(0.00013854938271604938) * t269 * t333 * v_sigma2 - t124 * t316 / f64x8::splat(24.0);
            let t341 = f64x8::splat(0.0033251851851851854) * t240 * t316 + t325 * t129 - t261 * t331 + t122 * t339;
            let t345 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t341));
            let tvsigma2 = t6 * t345;
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
