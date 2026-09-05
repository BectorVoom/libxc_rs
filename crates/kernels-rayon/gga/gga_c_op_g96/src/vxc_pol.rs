//! GGA_C_OP_G96 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`
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
pub fn gga_c_op_g96_vxc_pol(
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
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = ((t4).abs());
            let t11 = ((f64x8::splat(1.0) - t5).simd_le(zeta_threshold)) | (((v_rho0).simd_le(dens_threshold)) & ((v_rho1).simd_le(dens_threshold)));
            let t13 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t17 = -t14;
            let t18 = ((t13).select(t14, (t16).select(t17, t4)));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) - t19;
            let t21 = t20 * t2;
            let t24 = (f64x8::splat(2.0) * v_rho0 * t3).simd_le(zeta_threshold);
            let t27 = (f64x8::splat(2.0) * v_rho1 * t3).simd_le(zeta_threshold);
            let t28 = ((t24).select(t14, (t27).select(t17, t4)));
            let t29 = f64x8::splat(1.0) + t28;
            let t32 = (t29 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t33 = f64x8::splat(M_CBRT3);
            let t34 = t33 * t33;
            let t36 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t34 * t37;
            let t39 = f64x8::splat(M_CBRT4);
            let t40 = t38 * t39;
            let t41 = f64x8::splat(M_CBRT2);
            let t42 = (t29).simd_le(zeta_threshold);
            let t43 = f64x8::splat(1.0) - t28;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = ((t42).select(t14, (t44).select(t17, t28)));
            let t46 = f64x8::splat(1.0) + t45;
            let t47 = t46 * t2;
            let t48 = (simd::cbrt(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t51 = ((v_sigma0).sqrt());
            let t52 = (simd::cbrt(v_rho0));
            let t54 = f64x8::splat(1.0) / t52 / v_rho0;
            let t55 = t51 * t54;
            let t56 = ((t55).sqrt());
            let t57 = t56 * t55;
            let t61 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t38 * t39 * t57;
            let t62 = f64x8::splat(1.0) / t61;
            let t66 = ((t32).select(f64x8::splat(0.0), t40 * t41 * t49 * t62 / f64x8::splat(9.0)));
            let t70 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t71 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t72 = f64x8::splat(1.0) + t71;
            let t73 = t72 * t2;
            let t74 = (simd::cbrt(t73));
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = ((v_sigma2).sqrt());
            let t78 = (simd::cbrt(v_rho1));
            let t80 = f64x8::splat(1.0) / t78 / v_rho1;
            let t81 = t77 * t80;
            let t82 = ((t81).sqrt());
            let t83 = t82 * t81;
            let t87 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t38 * t39 * t83;
            let t88 = f64x8::splat(1.0) / t87;
            let t92 = ((t70).select(f64x8::splat(0.0), t40 * t41 * t75 * t88 / f64x8::splat(9.0)));
            let t93 = t66 + t92;
            let t94 = (t93).simd_eq(f64x8::splat(0.0));
            let t95 = ((t94).select(f64x8::splat(f64::EPSILON), t93));
            let t98 = f64x8::splat(3.59628532) / t95 + f64x8::splat(0.5764);
            let t99 = t95 * t95;
            let t100 = t99 * t99;
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = t99 * t95;
            let t104 = f64x8::splat(1.0) / t103;
            let t106 = f64x8::splat(1.0) / t99;
            let t108 = f64x8::splat(31.220719919544194) * t101 + f64x8::splat(14.903739892213245) * t104 + f64x8::splat(1.778517305052) * t106;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t98 * t109;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t110));
            acc_zk = tzk0;
            let t113 = t2 * t2;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t1 * t114;
            let t116 = t3 - t115;
            let t117 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t116)));
            let t118 = t18 * t117;
            let t119 = t2 * t98;
            let t120 = t119 * t109;
            let t123 = t20 * t98;
            let t125 = f64x8::splat(0.25) * t123 * t109;
            let t127 = f64x8::splat(1.0) / t48 / t47;
            let t128 = t41 * t127;
            let t129 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t116)));
            let t130 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t129)));
            let t132 = t130 * t2 + t45 + f64x8::splat(1.0);
            let t137 = t36 * t36;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t33 * t138;
            let t140 = t39 * t39;
            let t141 = t140 * t41;
            let t142 = t139 * t141;
            let t143 = t61 * t61;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t49 * t144;
            let t146 = t56 * t51;
            let t147 = v_rho0 * v_rho0;
            let t149 = f64x8::splat(1.0) / t52 / t147;
            let t150 = t146 * t149;
            let t155 = ((t32).select(f64x8::splat(0.0), -t40 * t128 * t62 * t132 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t142 * t145 * t150));
            let t157 = f64x8::splat(1.0) / t74 / t73;
            let t158 = t41 * t157;
            let t159 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t129)));
            let t161 = t159 * t2 + t71 + f64x8::splat(1.0);
            let t166 = ((t70).select(f64x8::splat(0.0), -t40 * t158 * t88 * t161 / f64x8::splat(27.0)));
            let t168 = ((t94).select(f64x8::splat(0.0), t155 + t166));
            let t169 = t106 * t168;
            let t170 = t169 * t109;
            let t173 = t108 * t108;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t98 * t174;
            let t177 = f64x8::splat(1.0) / t100 / t95;
            let t178 = t177 * t168;
            let t180 = t101 * t168;
            let t184 = -f64x8::splat(124.88287967817678) * t178 - f64x8::splat(44.711219676639736) * t180 - f64x8::splat(3.557034610104) * t104 * t168;
            let t185 = t175 * t184;
            let t189 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t118 * t120 - t125 + f64x8::splat(0.89907133) * t21 * t170 + f64x8::splat(0.25) * t21 * t185));
            let tvrho0 = t2 * t189 + tzk0;
            acc_vrho_0 = tvrho0;
            let t191 = -t3 - t115;
            let t192 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t191)));
            let t193 = t18 * t192;
            let t196 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t191)));
            let t197 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t196)));
            let t199 = t197 * t2 + t45 + f64x8::splat(1.0);
            let t204 = ((t32).select(f64x8::splat(0.0), -t40 * t128 * t62 * t199 / f64x8::splat(27.0)));
            let t205 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t196)));
            let t207 = t205 * t2 + t71 + f64x8::splat(1.0);
            let t212 = t87 * t87;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t75 * t213;
            let t215 = t82 * t77;
            let t216 = v_rho1 * v_rho1;
            let t218 = f64x8::splat(1.0) / t78 / t216;
            let t219 = t215 * t218;
            let t224 = ((t70).select(f64x8::splat(0.0), -t40 * t158 * t88 * t207 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t142 * t214 * t219));
            let t226 = ((t94).select(f64x8::splat(0.0), t204 + t224));
            let t227 = t106 * t226;
            let t228 = t227 * t109;
            let t231 = t177 * t226;
            let t233 = t101 * t226;
            let t235 = t104 * t226;
            let t237 = -f64x8::splat(124.88287967817678) * t231 - f64x8::splat(44.711219676639736) * t233 - f64x8::splat(3.557034610104) * t235;
            let t238 = t175 * t237;
            let t242 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t193 * t120 - t125 + f64x8::splat(0.89907133) * t21 * t228 + f64x8::splat(0.25) * t21 * t238));
            let tvrho1 = t2 * t242 + tzk0;
            acc_vrho_1 = tvrho1;
            let t244 = f64x8::splat(1.0) / t51;
            let t245 = t56 * t244;
            let t250 = ((t32).select(f64x8::splat(0.0), -t142 * t145 * t245 * t54 / f64x8::splat(2466.0)));
            let t251 = ((t94).select(f64x8::splat(0.0), t250));
            let t252 = t106 * t251;
            let t253 = t252 * t109;
            let t256 = t177 * t251;
            let t258 = t101 * t251;
            let t260 = t104 * t251;
            let t262 = -f64x8::splat(124.88287967817678) * t256 - f64x8::splat(44.711219676639736) * t258 - f64x8::splat(3.557034610104) * t260;
            let t263 = t175 * t262;
            let t267 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.89907133) * t21 * t253 + f64x8::splat(0.25) * t21 * t263));
            let tvsigma0 = t2 * t267;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t268 = f64x8::splat(1.0) / t77;
            let t269 = t82 * t268;
            let t274 = ((t70).select(f64x8::splat(0.0), -t142 * t214 * t269 * t80 / f64x8::splat(2466.0)));
            let t275 = ((t94).select(f64x8::splat(0.0), t274));
            let t276 = t106 * t275;
            let t277 = t276 * t109;
            let t280 = t177 * t275;
            let t282 = t101 * t275;
            let t284 = t104 * t275;
            let t286 = -f64x8::splat(124.88287967817678) * t280 - f64x8::splat(44.711219676639736) * t282 - f64x8::splat(3.557034610104) * t284;
            let t287 = t175 * t286;
            let t291 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.89907133) * t21 * t277 + f64x8::splat(0.25) * t21 * t287));
            let tvsigma2 = t2 * t291;
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
