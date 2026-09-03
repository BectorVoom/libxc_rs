//! MGGA_C_REVTPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revtpss.c`
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
pub fn mgga_c_revtpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_C0_c_0 = f64x8::splat(param_C0_c_0);
    let param_C0_c_1 = f64x8::splat(param_C0_c_1);
    let param_C0_c_2 = f64x8::splat(param_C0_c_2);
    let param_C0_c_3 = f64x8::splat(param_C0_c_3);
    let param_d = f64x8::splat(param_d);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t7 = (((f64x8::splat(0.0)).simd_lt(t5)).select(t5, -t5));
            let t8 = (-t7).simd_le(-f64x8::splat(0.999999999999));
            let t9 = param_C0_c_0;
            let t10 = param_C0_c_1;
            let t11 = param_C0_c_2;
            let t12 = param_C0_c_3;
            let t14 = t2 * t2;
            let t15 = t10 * t14;
            let t16 = t3 * t3;
            let t17 = f64x8::splat(1.0) / t16;
            let t19 = t14 * t14;
            let t20 = t11 * t19;
            let t21 = t16 * t16;
            let t22 = f64x8::splat(1.0) / t21;
            let t25 = t12 * t19 * t14;
            let t26 = t21 * t16;
            let t27 = f64x8::splat(1.0) / t26;
            let t29 = t15 * t17 + t20 * t22 + t25 * t27 + t9;
            let t30 = f64x8::splat(1.0) + t5;
            let t31 = (t30).simd_le(zeta_threshold);
            let t32 = zeta_threshold - f64x8::splat(1.0);
            let t33 = f64x8::splat(1.0) - t5;
            let t34 = (t33).simd_le(zeta_threshold);
            let t36 = ((t31).select(t32, (t34).select(-t32, t5)));
            let t37 = t36 * t36;
            let t38 = f64x8::splat(1.0) - t37;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t44 = v_sigma0 * t43;
            let t45 = f64x8::splat(1.0) + t36;
            let t46 = t45 / f64x8::splat(2.0);
            let t47 = (simd::cbrt(t46));
            let t48 = t47 * t47;
            let t49 = t48 * t46;
            let t51 = v_rho1 * v_rho1;
            let t52 = (simd::cbrt(v_rho1));
            let t53 = t52 * t52;
            let t55 = f64x8::splat(1.0) / t53 / t51;
            let t56 = v_sigma2 * t55;
            let t57 = f64x8::splat(1.0) - t36;
            let t58 = t57 / f64x8::splat(2.0);
            let t59 = (simd::cbrt(t58));
            let t60 = t59 * t59;
            let t61 = t60 * t58;
            let t64 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t65 = (simd::cbrt(t3));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t16;
            let t69 = t64 * t68;
            let t70 = t44 * t49 + t56 * t61 - t69;
            let t71 = t38 * t70;
            let t72 = f64x8::splat(M_CBRT3);
            let t73 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t74 = (simd::cbrt(t73));
            let t75 = t74 * t74;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t72 * t76;
            let t78 = (simd::cbrt(t45));
            let t79 = t78 * t45;
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = (simd::cbrt(t57));
            let t82 = t81 * t57;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t80 + t83;
            let t85 = t77 * t84;
            let t88 = f64x8::splat(1.0) + t71 * t85 / f64x8::splat(24.0);
            let t89 = t88 * t88;
            let t90 = t89 * t89;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = ((t8).select(t9 + t10 + t11 + t12, t29 * t91));
            let t94 = f64x8::splat(1.0) + t93;
            let t96 = f64x8::splat(1.0) / t41 / v_rho0;
            let t97 = v_tau0 * t96;
            let t98 = t30 / f64x8::splat(2.0);
            let t99 = (simd::cbrt(t98));
            let t100 = t99 * t99;
            let t101 = t100 * t98;
            let t104 = f64x8::splat(1.0) / t53 / v_rho1;
            let t105 = v_tau1 * t104;
            let t106 = t33 / f64x8::splat(2.0);
            let t107 = (simd::cbrt(t106));
            let t108 = t107 * t107;
            let t109 = t108 * t106;
            let t111 = t101 * t97 + t105 * t109;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = t69 * t112 / f64x8::splat(8.0);
            let t115 = (f64x8::splat(1.0)).simd_lt(t114);
            let t116 = ((t115).select(f64x8::splat(1.0), t114));
            let t117 = t116 * t116;
            let t118 = t94 * t117;
            let t120 = ((v_rho0).simd_le(dens_threshold)) | (t31);
            let t121 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t122 = (simd::cbrt(t121));
            let t123 = t72 * t122;
            let t124 = f64x8::splat(M_CBRT4);
            let t125 = t124 * t124;
            let t126 = f64x8::splat(1.0) / t65;
            let t127 = t125 * t126;
            let t128 = t123 * t127;
            let t130 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t128;
            let t131 = ((t128).sqrt());
            let t134 = ((t128) * (t128).sqrt());
            let t136 = t72 * t72;
            let t137 = t122 * t122;
            let t138 = t136 * t137;
            let t139 = f64x8::splat(1.0) / t66;
            let t140 = t124 * t139;
            let t141 = t138 * t140;
            let t143 = f64x8::splat(3.79785) * t131 + f64x8::splat(0.8969) * t128 + f64x8::splat(0.204775) * t134 + f64x8::splat(0.123235) * t141;
            let t146 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t143;
            let t147 = (simd::ln(t146));
            let t149 = f64x8::splat(0.0621814) * t130 * t147;
            let t150 = t37 * t37;
            let t151 = (t45).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(zeta_threshold));
            let t153 = t152 * zeta_threshold;
            let t154 = ((t151).select(t153, t79));
            let t155 = (t57).simd_le(zeta_threshold);
            let t156 = ((t155).select(t153, t82));
            let t157 = t154 + t156 - f64x8::splat(2.0);
            let t158 = t150 * t157;
            let t159 = f64x8::splat(M_CBRT2);
            let t162 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t159 - f64x8::splat(2.0));
            let t164 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t128;
            let t169 = f64x8::splat(7.05945) * t131 + f64x8::splat(1.549425) * t128 + f64x8::splat(0.420775) * t134 + f64x8::splat(0.1562925) * t141;
            let t172 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t169;
            let t173 = (simd::ln(t172));
            let t177 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t128;
            let t182 = f64x8::splat(5.1785) * t131 + f64x8::splat(0.905775) * t128 + f64x8::splat(0.1100325) * t134 + f64x8::splat(0.1241775) * t141;
            let t185 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t182;
            let t186 = (simd::ln(t185));
            let t187 = t177 * t186;
            let t189 = -f64x8::splat(0.0310907) * t164 * t173 + t149 - f64x8::splat(0.0197516734986138) * t187;
            let t190 = t162 * t189;
            let t191 = t158 * t190;
            let t192 = t157 * t162;
            let t194 = f64x8::splat(0.0197516734986138) * t192 * t187;
            let t195 = (simd::ln(f64x8::splat(2.0)));
            let t196 = f64x8::splat(1.0) - t195;
            let t197 = f64x8::splat(1.0) / t73;
            let t198 = t196 * t197;
            let t199 = t152 * t152;
            let t200 = t78 * t78;
            let t201 = ((t151).select(t199, t200));
            let t202 = t81 * t81;
            let t203 = ((t155).select(t199, t202));
            let t205 = t201 / f64x8::splat(2.0) + t203 / f64x8::splat(2.0);
            let t206 = t205 * t205;
            let t207 = t206 * t205;
            let t209 = f64x8::splat(1.0) + f64x8::splat(0.025) * t128;
            let t211 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t128;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = t209 * t212;
            let t215 = f64x8::splat(1.0) / t65 / t16;
            let t216 = t64 * t215;
            let t217 = t216 * t159;
            let t218 = f64x8::splat(1.0) / t206;
            let t220 = f64x8::splat(1.0) / t122;
            let t221 = t220 * t124;
            let t222 = t218 * t136 * t221;
            let t225 = f64x8::splat(1.0) / t196;
            let t227 = (-t149 + t191 + t194) * t225;
            let t228 = f64x8::splat(1.0) / t207;
            let t229 = t73 * t228;
            let t231 = (simd::exp(-t227 * t229));
            let t232 = t231 - f64x8::splat(1.0);
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t225 * t233;
            let t235 = t64 * t64;
            let t236 = t234 * t235;
            let t237 = t213 * t236;
            let t239 = f64x8::splat(1.0) / t66 / t21;
            let t240 = t159 * t159;
            let t241 = t239 * t240;
            let t242 = t206 * t206;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t241 * t243;
            let t245 = f64x8::splat(1.0) / t137;
            let t246 = t72 * t245;
            let t247 = t246 * t125;
            let t248 = t244 * t247;
            let t251 = t217 * t222 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t237 * t248;
            let t252 = t251 * t225;
            let t253 = t234 * t251;
            let t256 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t213 * t253;
            let t257 = f64x8::splat(1.0) / t256;
            let t258 = t252 * t257;
            let t261 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t213 * t258;
            let t262 = (simd::ln(t261));
            let t265 = t198 * t207 * t262 - t149 + t191 + t194;
            let t268 = t123 * t125;
            let t269 = t126 * t159;
            let t270 = f64x8::splat(1.0) / t45;
            let t271 = (simd::cbrt(t270));
            let t273 = t268 * t269 * t271;
            let t275 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t273;
            let t276 = ((t273).sqrt());
            let t279 = ((t273) * (t273).sqrt());
            let t281 = t138 * t124;
            let t282 = t139 * t240;
            let t283 = t271 * t271;
            let t285 = t281 * t282 * t283;
            let t287 = f64x8::splat(3.79785) * t276 + f64x8::splat(0.8969) * t273 + f64x8::splat(0.204775) * t279 + f64x8::splat(0.123235) * t285;
            let t290 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t287;
            let t291 = (simd::ln(t290));
            let t293 = f64x8::splat(0.0621814) * t275 * t291;
            let t294 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t296 = ((t294).select(t153, f64x8::splat(2.0) * t159));
            let t297 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t298 = ((t297).select(t153, f64x8::splat(0.0)));
            let t300 = (t296 + t298 - f64x8::splat(2.0)) * t162;
            let t302 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t273;
            let t307 = f64x8::splat(7.05945) * t276 + f64x8::splat(1.549425) * t273 + f64x8::splat(0.420775) * t279 + f64x8::splat(0.1562925) * t285;
            let t310 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t307;
            let t311 = (simd::ln(t310));
            let t315 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t273;
            let t320 = f64x8::splat(5.1785) * t276 + f64x8::splat(0.905775) * t273 + f64x8::splat(0.1100325) * t279 + f64x8::splat(0.1241775) * t285;
            let t323 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t320;
            let t324 = (simd::ln(t323));
            let t325 = t315 * t324;
            let t328 = t300 * (-f64x8::splat(0.0310907) * t302 * t311 + t293 - f64x8::splat(0.0197516734986138) * t325);
            let t330 = f64x8::splat(0.0197516734986138) * t300 * t325;
            let t331 = ((t294).select(t199, t240));
            let t332 = ((t297).select(t199, f64x8::splat(0.0)));
            let t334 = t331 / f64x8::splat(2.0) + t332 / f64x8::splat(2.0);
            let t335 = t334 * t334;
            let t336 = t335 * t334;
            let t338 = f64x8::splat(1.0) + f64x8::splat(0.025) * t273;
            let t340 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t273;
            let t341 = f64x8::splat(1.0) / t340;
            let t342 = t338 * t341;
            let t343 = f64x8::splat(1.0) / t335;
            let t344 = t343 * t136;
            let t345 = t44 * t344;
            let t346 = f64x8::splat(1.0) / t271;
            let t348 = t221 * t65 * t346;
            let t351 = t342 * t225;
            let t354 = f64x8::splat(1.0) / t336;
            let t355 = t73 * t354;
            let t357 = (simd::exp(-(-t293 + t328 + t330) * t225 * t355));
            let t358 = t357 - f64x8::splat(1.0);
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = v_sigma0 * v_sigma0;
            let t361 = t359 * t360;
            let t362 = t39 * t39;
            let t363 = t362 * v_rho0;
            let t365 = f64x8::splat(1.0) / t40 / t363;
            let t366 = t361 * t365;
            let t367 = t351 * t366;
            let t368 = t335 * t335;
            let t369 = f64x8::splat(1.0) / t368;
            let t370 = t369 * t72;
            let t371 = t370 * t245;
            let t372 = t125 * t66;
            let t373 = f64x8::splat(1.0) / t283;
            let t375 = t371 * t372 * t373;
            let t378 = t345 * t348 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t367 * t375;
            let t379 = t378 * t225;
            let t380 = t225 * t359;
            let t381 = t380 * t378;
            let t384 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t342 * t381;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t379 * t385;
            let t389 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t342 * t386;
            let t390 = (simd::ln(t389));
            let t393 = t198 * t336 * t390 - t293 + t328 + t330;
            let t394 = (t265).simd_lt(t393);
            let t395 = ((t394).select(t393, t265));
            let t398 = ((t120).select(t265 * t30 / f64x8::splat(2.0), t395 * t45 / f64x8::splat(2.0)));
            let t400 = ((v_rho1).simd_le(dens_threshold)) | (t34);
            let t403 = f64x8::splat(1.0) / t57;
            let t404 = (simd::cbrt(t403));
            let t406 = t268 * t269 * t404;
            let t408 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t406;
            let t409 = ((t406).sqrt());
            let t412 = ((t406) * (t406).sqrt());
            let t414 = t404 * t404;
            let t416 = t281 * t282 * t414;
            let t418 = f64x8::splat(3.79785) * t409 + f64x8::splat(0.8969) * t406 + f64x8::splat(0.204775) * t412 + f64x8::splat(0.123235) * t416;
            let t421 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t418;
            let t422 = (simd::ln(t421));
            let t424 = f64x8::splat(0.0621814) * t408 * t422;
            let t426 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t406;
            let t431 = f64x8::splat(7.05945) * t409 + f64x8::splat(1.549425) * t406 + f64x8::splat(0.420775) * t412 + f64x8::splat(0.1562925) * t416;
            let t434 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t431;
            let t435 = (simd::ln(t434));
            let t439 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t406;
            let t444 = f64x8::splat(5.1785) * t409 + f64x8::splat(0.905775) * t406 + f64x8::splat(0.1100325) * t412 + f64x8::splat(0.1241775) * t416;
            let t447 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t444;
            let t448 = (simd::ln(t447));
            let t449 = t439 * t448;
            let t452 = t300 * (-f64x8::splat(0.0310907) * t426 * t435 + t424 - f64x8::splat(0.0197516734986138) * t449);
            let t454 = f64x8::splat(0.0197516734986138) * t300 * t449;
            let t456 = f64x8::splat(1.0) + f64x8::splat(0.025) * t406;
            let t458 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t406;
            let t459 = f64x8::splat(1.0) / t458;
            let t460 = t456 * t459;
            let t461 = t56 * t344;
            let t462 = f64x8::splat(1.0) / t404;
            let t464 = t221 * t65 * t462;
            let t467 = t460 * t225;
            let t471 = (simd::exp(-(-t424 + t452 + t454) * t225 * t355));
            let t472 = t471 - f64x8::splat(1.0);
            let t473 = f64x8::splat(1.0) / t472;
            let t474 = v_sigma2 * v_sigma2;
            let t475 = t473 * t474;
            let t476 = t51 * t51;
            let t477 = t476 * v_rho1;
            let t479 = f64x8::splat(1.0) / t52 / t477;
            let t480 = t475 * t479;
            let t481 = t467 * t480;
            let t482 = f64x8::splat(1.0) / t414;
            let t484 = t371 * t372 * t482;
            let t487 = t461 * t464 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t481 * t484;
            let t488 = t487 * t225;
            let t489 = t225 * t473;
            let t490 = t489 * t487;
            let t493 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t460 * t490;
            let t494 = f64x8::splat(1.0) / t493;
            let t495 = t488 * t494;
            let t498 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t460 * t495;
            let t499 = (simd::ln(t498));
            let t502 = t198 * t336 * t499 - t424 + t452 + t454;
            let t503 = (t265).simd_lt(t502);
            let t504 = ((t503).select(t502, t265));
            let t507 = ((t400).select(t265 * t33 / f64x8::splat(2.0), t504 * t57 / f64x8::splat(2.0)));
            let t508 = t398 + t507;
            let t511 = t117 * t93 + f64x8::splat(1.0);
            let t512 = t19 * t22;
            let t513 = (simd::cbrt(t30));
            let t514 = t513 * t30;
            let t515 = ((t31).select(t153, t514));
            let t516 = (simd::cbrt(t33));
            let t517 = t516 * t33;
            let t518 = ((t34).select(t153, t517));
            let t519 = t515 + t518 - f64x8::splat(2.0);
            let t520 = t519 * t162;
            let t521 = t520 * t189;
            let t522 = t512 * t521;
            let t524 = f64x8::splat(0.0197516734986138) * t520 * t187;
            let t525 = t513 * t513;
            let t526 = ((t31).select(t199, t525));
            let t527 = t516 * t516;
            let t528 = ((t34).select(t199, t527));
            let t530 = t526 / f64x8::splat(2.0) + t528 / f64x8::splat(2.0);
            let t531 = t530 * t530;
            let t532 = t531 * t530;
            let t533 = f64x8::splat(1.0) / t531;
            let t535 = t533 * t136 * t221;
            let t539 = (-t149 + t522 + t524) * t225;
            let t540 = f64x8::splat(1.0) / t532;
            let t541 = t73 * t540;
            let t543 = (simd::exp(-t539 * t541));
            let t544 = t543 - f64x8::splat(1.0);
            let t545 = f64x8::splat(1.0) / t544;
            let t546 = t225 * t545;
            let t547 = t546 * t235;
            let t548 = t213 * t547;
            let t549 = t531 * t531;
            let t550 = f64x8::splat(1.0) / t549;
            let t551 = t241 * t550;
            let t552 = t551 * t247;
            let t555 = t217 * t535 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t548 * t552;
            let t556 = t555 * t225;
            let t557 = t546 * t555;
            let t560 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t213 * t557;
            let t561 = f64x8::splat(1.0) / t560;
            let t562 = t556 * t561;
            let t565 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t213 * t562;
            let t566 = (simd::ln(t565));
            let t569 = t198 * t532 * t566 - t149 + t522 + t524;
            let t571 = -t118 * t508 + t511 * t569;
            let t572 = param_d * t571;
            let t573 = t117 * t116;
            let t575 = t572 * t573 + f64x8::splat(1.0);
            let tzk0 = t571 * t575;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
