//! HYB_LDA_XC_BN05 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn hyb_lda_xc_bn05_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t3 * t1;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t4;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = v_rho0 - v_rho1;
            let t11 = v_rho0 + v_rho1;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t12 * t10;
            let t14 = f64x8::splat(1.0) + t13;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t14));
            let t20 = ((t15).select(t17, t18 * t14));
            let t21 = t20 * t9;
            let t22 = (simd::cbrt(t11));
            let t23 = (simd::cbrt(f64x8::splat(9.0)));
            let t24 = t23 * t23;
            let t25 = t3 * t3;
            let t26 = t25 * t24;
            let t27 = param_hyb_omega_0 * t26;
            let t28 = f64x8::splat(1.0) / t22;
            let t29 = t28 * t1;
            let t30 = ((t15).select(t16, t18));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t31 * t29 * t27 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.92)).simd_le(t34);
            let t36 = (f64x8::splat(1.92)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.92)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t50 = t47 * t38;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = t47 * t41;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t47 * t44;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = f64x8::splat(1.0) / t59 / t38;
            let t66 = f64x8::splat(1.0) / t59 / t41;
            let t69 = f64x8::splat(1.0) / t59 / t44;
            let t72 = f64x8::splat(1.0) / t59 / t47;
            let t75 = f64x8::splat(1.0) / t59 / t50;
            let t78 = f64x8::splat(1.0) / t59 / t53;
            let t81 = f64x8::splat(1.0) / t59 / t56;
            let t83 = t59 * t59;
            let t84 = f64x8::splat(1.0) / t83;
            let t87 = f64x8::splat(1.0) / t83 / t38;
            let t90 = f64x8::splat(1.0) / t83 / t41;
            let t92 = f64x8::splat(1.0) / t38 / f64x8::splat(9.0) - t42 / f64x8::splat(30.0) + t45 / f64x8::splat(70.0) - t48 / f64x8::splat(135.0) + t51 / f64x8::splat(231.0) - t54 / f64x8::splat(364.0) + t57 / f64x8::splat(540.0) - t60 / f64x8::splat(765.0) + t63 / f64x8::splat(1045.0) - t66 / f64x8::splat(1386.0) + t69 / f64x8::splat(1794.0) - t72 / f64x8::splat(2275.0) + t75 / f64x8::splat(2835.0) - t78 / f64x8::splat(3480.0) + t81 / f64x8::splat(4216.0) - t84 / f64x8::splat(5049.0) + t87 / f64x8::splat(5985.0) - t90 / f64x8::splat(7030.0);
            let t93 = ((t36).select(f64x8::splat(1.92), t34));
            let t94 = (simd::atan2(f64x8::splat(1.0), t93));
            let t95 = t93 * t93;
            let t96 = t95 + f64x8::splat(3.0);
            let t97 = f64x8::splat(1.0) / t95;
            let t98 = f64x8::splat(1.0) + t97;
            let t99 = (simd::ln(t98));
            let t101 = -t96 * t99 + f64x8::splat(1.0);
            let t104 = t94 + t101 * t93 / f64x8::splat(4.0);
            let t108 = ((t35).select(t92, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t93));
            let t109 = t108 * t22;
            let t112 = f64x8::splat(3.0) / f64x8::splat(32.0) * t109 * t21 * t7;
            let t113 = f64x8::splat(1.0) - t13;
            let t114 = (t113).simd_le(zeta_threshold);
            let t115 = (simd::cbrt(t113));
            let t117 = ((t114).select(t17, t115 * t113));
            let t118 = t117 * t9;
            let t119 = ((t114).select(t16, t115));
            let t120 = f64x8::splat(1.0) / t119;
            let t123 = t120 * t29 * t27 / f64x8::splat(18.0);
            let t124 = (f64x8::splat(1.92)).simd_le(t123);
            let t125 = (f64x8::splat(1.92)).simd_lt(t123);
            let t126 = ((t125).select(t123, f64x8::splat(1.92)));
            let t127 = t126 * t126;
            let t130 = t127 * t127;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t130 * t127;
            let t134 = f64x8::splat(1.0) / t133;
            let t136 = t130 * t130;
            let t137 = f64x8::splat(1.0) / t136;
            let t139 = t136 * t127;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = t136 * t130;
            let t143 = f64x8::splat(1.0) / t142;
            let t145 = t136 * t133;
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = t136 * t136;
            let t149 = f64x8::splat(1.0) / t148;
            let t152 = f64x8::splat(1.0) / t148 / t127;
            let t155 = f64x8::splat(1.0) / t148 / t130;
            let t158 = f64x8::splat(1.0) / t148 / t133;
            let t161 = f64x8::splat(1.0) / t148 / t136;
            let t164 = f64x8::splat(1.0) / t148 / t139;
            let t167 = f64x8::splat(1.0) / t148 / t142;
            let t170 = f64x8::splat(1.0) / t148 / t145;
            let t172 = t148 * t148;
            let t173 = f64x8::splat(1.0) / t172;
            let t176 = f64x8::splat(1.0) / t172 / t127;
            let t179 = f64x8::splat(1.0) / t172 / t130;
            let t181 = f64x8::splat(1.0) / t127 / f64x8::splat(9.0) - t131 / f64x8::splat(30.0) + t134 / f64x8::splat(70.0) - t137 / f64x8::splat(135.0) + t140 / f64x8::splat(231.0) - t143 / f64x8::splat(364.0) + t146 / f64x8::splat(540.0) - t149 / f64x8::splat(765.0) + t152 / f64x8::splat(1045.0) - t155 / f64x8::splat(1386.0) + t158 / f64x8::splat(1794.0) - t161 / f64x8::splat(2275.0) + t164 / f64x8::splat(2835.0) - t167 / f64x8::splat(3480.0) + t170 / f64x8::splat(4216.0) - t173 / f64x8::splat(5049.0) + t176 / f64x8::splat(5985.0) - t179 / f64x8::splat(7030.0);
            let t182 = ((t125).select(f64x8::splat(1.92), t123));
            let t183 = (simd::atan2(f64x8::splat(1.0), t182));
            let t184 = t182 * t182;
            let t185 = t184 + f64x8::splat(3.0);
            let t186 = f64x8::splat(1.0) / t184;
            let t187 = f64x8::splat(1.0) + t186;
            let t188 = (simd::ln(t187));
            let t190 = -t185 * t188 + f64x8::splat(1.0);
            let t193 = t183 + t190 * t182 / f64x8::splat(4.0);
            let t197 = ((t124).select(t181, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t182));
            let t198 = t197 * t22;
            let t201 = f64x8::splat(3.0) / f64x8::splat(32.0) * t198 * t118 * t7;
            let t203 = t28 * t6 * t4;
            let t205 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t203;
            let t206 = ((t203).sqrt());
            let t209 = ((t203) * (t203).sqrt());
            let t211 = t1 * t1;
            let t212 = t25 * t211;
            let t213 = t22 * t22;
            let t214 = f64x8::splat(1.0) / t213;
            let t216 = t214 * t5 * t212;
            let t218 = f64x8::splat(3.79785) * t206 + f64x8::splat(0.8969) * t203 + f64x8::splat(0.204775) * t209 + f64x8::splat(0.123235) * t216;
            let t221 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t218;
            let t222 = (simd::ln(t221));
            let t224 = f64x8::splat(0.0621814) * t222 * t205;
            let t225 = t10 * t10;
            let t226 = t225 * t225;
            let t227 = t11 * t11;
            let t228 = t227 * t227;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t226;
            let t231 = t20 + t117 - f64x8::splat(2.0);
            let t234 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t8 - f64x8::splat(2.0));
            let t235 = t234 * t231;
            let t237 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t203;
            let t242 = f64x8::splat(7.05945) * t206 + f64x8::splat(1.549425) * t203 + f64x8::splat(0.420775) * t209 + f64x8::splat(0.1562925) * t216;
            let t245 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t242;
            let t246 = (simd::ln(t245));
            let t250 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t203;
            let t255 = f64x8::splat(5.1785) * t206 + f64x8::splat(0.905775) * t203 + f64x8::splat(0.1100325) * t209 + f64x8::splat(0.1241775) * t216;
            let t258 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t255;
            let t259 = (simd::ln(t258));
            let t260 = t259 * t250;
            let t262 = -f64x8::splat(0.0310907) * t246 * t237 + t224 - f64x8::splat(0.0197516734986138) * t260;
            let t263 = t262 * t235;
            let t267 = -t224 + t263 * t230 + f64x8::splat(0.0197516734986138) * t260 * t235;
            let t270 = f64x8::splat(3.2) - f64x8::splat(0.225) * t203 + t216 / f64x8::splat(4.0);
            let t271 = f64x8::splat(1.0) / t270;
            let t273 = f64x8::splat(3.4602) * t271 * t267;
            let tzk0 = -t112 - t201 + t273;
            acc_zk = tzk0;
            let t274 = f64x8::splat(1.0) / t227;
            let t275 = t274 * t10;
            let t276 = t12 - t275;
            let t279 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t276 * t18));
            let t280 = t279 * t9;
            let t282 = t109 * t280 * t7;
            let t283 = f64x8::splat(3.0) / f64x8::splat(32.0) * t282;
            let t284 = t108 * t214;
            let t286 = t284 * t21 * t7;
            let t287 = t286 / f64x8::splat(32.0);
            let t288 = t38 * t37;
            let t289 = f64x8::splat(1.0) / t288;
            let t291 = f64x8::splat(1.0) / t22 / t11;
            let t292 = t291 * t1;
            let t295 = t31 * t292 * t27 / f64x8::splat(54.0);
            let t296 = t30 * t30;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t18 * t18;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t276 * t299;
            let t302 = ((t15).select(f64x8::splat(0.0), t300 / f64x8::splat(3.0)));
            let t303 = t302 * t297;
            let t307 = -t295 - t303 * t29 * t27 / f64x8::splat(18.0);
            let t308 = ((t36).select(t307, f64x8::splat(0.0)));
            let t311 = t41 * t37;
            let t312 = f64x8::splat(1.0) / t311;
            let t315 = t41 * t288;
            let t316 = f64x8::splat(1.0) / t315;
            let t319 = t47 * t37;
            let t320 = f64x8::splat(1.0) / t319;
            let t323 = t47 * t288;
            let t324 = f64x8::splat(1.0) / t323;
            let t327 = t47 * t311;
            let t328 = f64x8::splat(1.0) / t327;
            let t331 = t47 * t315;
            let t332 = f64x8::splat(1.0) / t331;
            let t336 = f64x8::splat(1.0) / t59 / t37;
            let t340 = f64x8::splat(1.0) / t59 / t288;
            let t344 = f64x8::splat(1.0) / t59 / t311;
            let t348 = f64x8::splat(1.0) / t59 / t315;
            let t352 = f64x8::splat(1.0) / t59 / t319;
            let t356 = f64x8::splat(1.0) / t59 / t323;
            let t360 = f64x8::splat(1.0) / t59 / t327;
            let t364 = f64x8::splat(1.0) / t59 / t331;
            let t368 = f64x8::splat(1.0) / t83 / t37;
            let t372 = f64x8::splat(1.0) / t83 / t288;
            let t376 = f64x8::splat(1.0) / t83 / t311;
            let t379 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t308 * t289 + f64x8::splat(2.0) / f64x8::splat(15.0) * t308 * t312 - f64x8::splat(3.0) / f64x8::splat(35.0) * t308 * t316 + f64x8::splat(8.0) / f64x8::splat(135.0) * t308 * t320 - f64x8::splat(10.0) / f64x8::splat(231.0) * t308 * t324 + f64x8::splat(3.0) / f64x8::splat(91.0) * t308 * t328 - f64x8::splat(7.0) / f64x8::splat(270.0) * t308 * t332 + f64x8::splat(16.0) / f64x8::splat(765.0) * t308 * t336 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t308 * t340 + f64x8::splat(10.0) / f64x8::splat(693.0) * t308 * t344 - f64x8::splat(11.0) / f64x8::splat(897.0) * t308 * t348 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t308 * t352 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t308 * t356 + f64x8::splat(7.0) / f64x8::splat(870.0) * t308 * t360 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t308 * t364 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t308 * t368 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t308 * t372 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t308 * t376;
            let t380 = ((t36).select(f64x8::splat(0.0), t307));
            let t383 = f64x8::splat(1.0) / t98;
            let t389 = t95 * t93;
            let t390 = f64x8::splat(1.0) / t389;
            let t391 = t390 * t96;
            let t392 = t383 * t380;
            let t395 = -f64x8::splat(2.0) * t380 * t93 * t99 + f64x8::splat(2.0) * t391 * t392;
            let t398 = -t383 * t97 * t380 + t101 * t380 / f64x8::splat(4.0) + t395 * t93 / f64x8::splat(4.0);
            let t402 = ((t35).select(t379, -f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t380 - f64x8::splat(8.0) / f64x8::splat(3.0) * t398 * t93));
            let t403 = t402 * t22;
            let t405 = t403 * t21 * t7;
            let t406 = f64x8::splat(3.0) / f64x8::splat(32.0) * t405;
            let t407 = -t276;
            let t410 = ((t114).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t407 * t115));
            let t411 = t410 * t9;
            let t413 = t198 * t411 * t7;
            let t414 = f64x8::splat(3.0) / f64x8::splat(32.0) * t413;
            let t415 = t197 * t214;
            let t417 = t415 * t118 * t7;
            let t418 = t417 / f64x8::splat(32.0);
            let t419 = t127 * t126;
            let t420 = f64x8::splat(1.0) / t419;
            let t423 = t120 * t292 * t27 / f64x8::splat(54.0);
            let t424 = t119 * t119;
            let t425 = f64x8::splat(1.0) / t424;
            let t426 = t115 * t115;
            let t427 = f64x8::splat(1.0) / t426;
            let t428 = t407 * t427;
            let t430 = ((t114).select(f64x8::splat(0.0), t428 / f64x8::splat(3.0)));
            let t431 = t430 * t425;
            let t435 = -t423 - t431 * t29 * t27 / f64x8::splat(18.0);
            let t436 = ((t125).select(t435, f64x8::splat(0.0)));
            let t439 = t130 * t126;
            let t440 = f64x8::splat(1.0) / t439;
            let t443 = t130 * t419;
            let t444 = f64x8::splat(1.0) / t443;
            let t447 = t136 * t126;
            let t448 = f64x8::splat(1.0) / t447;
            let t451 = t136 * t419;
            let t452 = f64x8::splat(1.0) / t451;
            let t455 = t136 * t439;
            let t456 = f64x8::splat(1.0) / t455;
            let t459 = t136 * t443;
            let t460 = f64x8::splat(1.0) / t459;
            let t464 = f64x8::splat(1.0) / t148 / t126;
            let t468 = f64x8::splat(1.0) / t148 / t419;
            let t472 = f64x8::splat(1.0) / t148 / t439;
            let t476 = f64x8::splat(1.0) / t148 / t443;
            let t480 = f64x8::splat(1.0) / t148 / t447;
            let t484 = f64x8::splat(1.0) / t148 / t451;
            let t488 = f64x8::splat(1.0) / t148 / t455;
            let t492 = f64x8::splat(1.0) / t148 / t459;
            let t496 = f64x8::splat(1.0) / t172 / t126;
            let t500 = f64x8::splat(1.0) / t172 / t419;
            let t504 = f64x8::splat(1.0) / t172 / t439;
            let t507 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t436 * t420 + f64x8::splat(2.0) / f64x8::splat(15.0) * t436 * t440 - f64x8::splat(3.0) / f64x8::splat(35.0) * t436 * t444 + f64x8::splat(8.0) / f64x8::splat(135.0) * t436 * t448 - f64x8::splat(10.0) / f64x8::splat(231.0) * t436 * t452 + f64x8::splat(3.0) / f64x8::splat(91.0) * t436 * t456 - f64x8::splat(7.0) / f64x8::splat(270.0) * t436 * t460 + f64x8::splat(16.0) / f64x8::splat(765.0) * t436 * t464 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t436 * t468 + f64x8::splat(10.0) / f64x8::splat(693.0) * t436 * t472 - f64x8::splat(11.0) / f64x8::splat(897.0) * t436 * t476 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t436 * t480 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t436 * t484 + f64x8::splat(7.0) / f64x8::splat(870.0) * t436 * t488 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t436 * t492 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t436 * t496 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t436 * t500 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t436 * t504;
            let t508 = ((t125).select(f64x8::splat(0.0), t435));
            let t511 = f64x8::splat(1.0) / t187;
            let t517 = t184 * t182;
            let t518 = f64x8::splat(1.0) / t517;
            let t519 = t518 * t185;
            let t520 = t511 * t508;
            let t523 = -f64x8::splat(2.0) * t182 * t188 * t508 + f64x8::splat(2.0) * t519 * t520;
            let t526 = -t511 * t186 * t508 + t190 * t508 / f64x8::splat(4.0) + t523 * t182 / f64x8::splat(4.0);
            let t530 = ((t124).select(t507, -f64x8::splat(8.0) / f64x8::splat(3.0) * t526 * t182 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t508));
            let t531 = t530 * t22;
            let t533 = t531 * t118 * t7;
            let t534 = f64x8::splat(3.0) / f64x8::splat(32.0) * t533;
            let t535 = t291 * t6;
            let t538 = f64x8::splat(0.0011073470983333333) * t222 * t535 * t4;
            let t539 = t218 * t218;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t540 * t205;
            let t543 = t1 / t206;
            let t544 = t6 * t3;
            let t545 = t291 * t544;
            let t546 = t545 * t543;
            let t548 = t535 * t4;
            let t550 = ((t203).sqrt());
            let t551 = t1 * t550;
            let t552 = t545 * t551;
            let t555 = f64x8::splat(1.0) / t213 / t11;
            let t557 = t555 * t5 * t212;
            let t559 = -f64x8::splat(0.632975) * t546 - f64x8::splat(0.29896666666666666) * t548 - f64x8::splat(0.1023875) * t552 - f64x8::splat(0.08215666666666667) * t557;
            let t560 = f64x8::splat(1.0) / t221;
            let t561 = t560 * t559;
            let t563 = f64x8::splat(1.0) * t561 * t541;
            let t564 = t225 * t10;
            let t565 = t229 * t564;
            let t567 = f64x8::splat(4.0) * t263 * t565;
            let t568 = t228 * t11;
            let t569 = f64x8::splat(1.0) / t568;
            let t570 = t569 * t226;
            let t572 = f64x8::splat(4.0) * t263 * t570;
            let t574 = t234 * (t279 + t410);
            let t575 = t262 * t574;
            let t580 = t242 * t242;
            let t581 = f64x8::splat(1.0) / t580;
            let t582 = t581 * t237;
            let t587 = -f64x8::splat(1.176575) * t546 - f64x8::splat(0.516475) * t548 - f64x8::splat(0.2103875) * t552 - f64x8::splat(0.104195) * t557;
            let t588 = f64x8::splat(1.0) / t245;
            let t589 = t588 * t587;
            let t595 = t255 * t255;
            let t596 = f64x8::splat(1.0) / t595;
            let t597 = t596 * t250;
            let t602 = -f64x8::splat(0.8630833333333333) * t546 - f64x8::splat(0.301925) * t548 - f64x8::splat(0.05501625) * t552 - f64x8::splat(0.082785) * t557;
            let t603 = f64x8::splat(1.0) / t258;
            let t604 = t603 * t602;
            let t607 = f64x8::splat(0.0005323764196666666) * t246 * t535 * t4 + f64x8::splat(1.0) * t589 * t582 - t538 - t563 + f64x8::splat(0.00018311447306006544) * t259 * t535 * t4 + f64x8::splat(0.5848223622634646) * t604 * t597;
            let t608 = t607 * t235;
            let t609 = t608 * t230;
            let t612 = t1 * t235;
            let t614 = t259 * t291 * t544;
            let t616 = f64x8::splat(0.00018311447306006544) * t614 * t612;
            let t617 = t250 * t235;
            let t619 = t603 * t602 * t596;
            let t621 = f64x8::splat(0.5848223622634646) * t619 * t617;
            let t622 = t538 + t563 + t567 - t572 + t575 * t230 + t609 + f64x8::splat(0.0197516734986138) * t260 * t574 - t616 - t621;
            let t623 = t271 * t622;
            let t624 = f64x8::splat(3.4602) * t623;
            let t625 = t270 * t270;
            let t626 = f64x8::splat(1.0) / t625;
            let t627 = t626 * t267;
            let t630 = f64x8::splat(0.075) * t548 - t557 / f64x8::splat(6.0);
            let t631 = t630 * t627;
            let t632 = f64x8::splat(3.4602) * t631;
            let tvrho0 = -t112 - t201 + t273 + (-t283 - t287 - t406 - t414 - t418 - t534 + t624 - t632) * t11;
            acc_vrho_0 = tvrho0;
            let t635 = -t12 - t275;
            let t638 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t635 * t18));
            let t639 = t638 * t9;
            let t641 = t109 * t639 * t7;
            let t642 = f64x8::splat(3.0) / f64x8::splat(32.0) * t641;
            let t643 = t635 * t299;
            let t645 = ((t15).select(f64x8::splat(0.0), t643 / f64x8::splat(3.0)));
            let t646 = t645 * t297;
            let t650 = -t295 - t646 * t29 * t27 / f64x8::splat(18.0);
            let t651 = ((t36).select(t650, f64x8::splat(0.0)));
            let t654 = t651 * t312;
            let t656 = t651 * t316;
            let t658 = t651 * t320;
            let t660 = t651 * t324;
            let t662 = t651 * t328;
            let t664 = t651 * t332;
            let t666 = t651 * t336;
            let t668 = t651 * t340;
            let t670 = t651 * t344;
            let t672 = t651 * t348;
            let t674 = t651 * t352;
            let t676 = t651 * t356;
            let t678 = t651 * t360;
            let t680 = t651 * t364;
            let t682 = t651 * t368;
            let t684 = t651 * t372;
            let t686 = t651 * t376;
            let t688 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t651 * t289 + f64x8::splat(2.0) / f64x8::splat(15.0) * t654 - f64x8::splat(3.0) / f64x8::splat(35.0) * t656 + f64x8::splat(8.0) / f64x8::splat(135.0) * t658 - f64x8::splat(10.0) / f64x8::splat(231.0) * t660 + f64x8::splat(3.0) / f64x8::splat(91.0) * t662 - f64x8::splat(7.0) / f64x8::splat(270.0) * t664 + f64x8::splat(16.0) / f64x8::splat(765.0) * t666 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t668 + f64x8::splat(10.0) / f64x8::splat(693.0) * t670 - f64x8::splat(11.0) / f64x8::splat(897.0) * t672 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t674 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t676 + f64x8::splat(7.0) / f64x8::splat(870.0) * t678 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t680 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t682 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t684 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t686;
            let t689 = ((t36).select(f64x8::splat(0.0), t650));
            let t691 = t97 * t689;
            let t697 = t383 * t689;
            let t700 = -f64x8::splat(2.0) * t689 * t93 * t99 + f64x8::splat(2.0) * t391 * t697;
            let t703 = -t383 * t691 + t101 * t689 / f64x8::splat(4.0) + t700 * t93 / f64x8::splat(4.0);
            let t707 = ((t35).select(t688, -f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t689 - f64x8::splat(8.0) / f64x8::splat(3.0) * t703 * t93));
            let t708 = t707 * t22;
            let t710 = t708 * t21 * t7;
            let t711 = f64x8::splat(3.0) / f64x8::splat(32.0) * t710;
            let t712 = -t635;
            let t715 = ((t114).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t712 * t115));
            let t716 = t715 * t9;
            let t718 = t198 * t716 * t7;
            let t719 = f64x8::splat(3.0) / f64x8::splat(32.0) * t718;
            let t720 = t712 * t427;
            let t722 = ((t114).select(f64x8::splat(0.0), t720 / f64x8::splat(3.0)));
            let t723 = t722 * t425;
            let t727 = -t423 - t723 * t29 * t27 / f64x8::splat(18.0);
            let t728 = ((t125).select(t727, f64x8::splat(0.0)));
            let t731 = t728 * t440;
            let t733 = t728 * t444;
            let t735 = t728 * t448;
            let t737 = t728 * t452;
            let t739 = t728 * t456;
            let t741 = t728 * t460;
            let t743 = t728 * t464;
            let t745 = t728 * t468;
            let t747 = t728 * t472;
            let t749 = t728 * t476;
            let t751 = t728 * t480;
            let t753 = t728 * t484;
            let t755 = t728 * t488;
            let t757 = t728 * t492;
            let t759 = t728 * t496;
            let t761 = t728 * t500;
            let t763 = t728 * t504;
            let t765 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t728 * t420 + f64x8::splat(2.0) / f64x8::splat(15.0) * t731 - f64x8::splat(3.0) / f64x8::splat(35.0) * t733 + f64x8::splat(8.0) / f64x8::splat(135.0) * t735 - f64x8::splat(10.0) / f64x8::splat(231.0) * t737 + f64x8::splat(3.0) / f64x8::splat(91.0) * t739 - f64x8::splat(7.0) / f64x8::splat(270.0) * t741 + f64x8::splat(16.0) / f64x8::splat(765.0) * t743 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t745 + f64x8::splat(10.0) / f64x8::splat(693.0) * t747 - f64x8::splat(11.0) / f64x8::splat(897.0) * t749 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t751 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t753 + f64x8::splat(7.0) / f64x8::splat(870.0) * t755 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t757 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t759 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t761 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t763;
            let t766 = ((t125).select(f64x8::splat(0.0), t727));
            let t768 = t186 * t766;
            let t774 = t511 * t766;
            let t777 = -f64x8::splat(2.0) * t182 * t188 * t766 + f64x8::splat(2.0) * t519 * t774;
            let t780 = -t511 * t768 + t190 * t766 / f64x8::splat(4.0) + t777 * t182 / f64x8::splat(4.0);
            let t784 = ((t124).select(t765, -f64x8::splat(8.0) / f64x8::splat(3.0) * t780 * t182 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t766));
            let t785 = t784 * t22;
            let t787 = t785 * t118 * t7;
            let t788 = f64x8::splat(3.0) / f64x8::splat(32.0) * t787;
            let t790 = t234 * (t638 + t715);
            let t791 = t262 * t790;
            let t795 = t538 + t563 - t567 - t572 + t791 * t230 + t609 + f64x8::splat(0.0197516734986138) * t260 * t790 - t616 - t621;
            let t796 = t271 * t795;
            let t797 = f64x8::splat(3.4602) * t796;
            let tvrho1 = -t112 - t201 + t273 + (-t642 - t287 - t711 - t719 - t418 - t788 + t797 - t632) * t11;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
