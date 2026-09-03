//! GGA_C_PBE_VWN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`
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
pub fn gga_c_pbe_vwn_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = ((t11).sqrt());
            let t15 = t12 + f64x8::splat(1.86372) * t13 + f64x8::splat(12.9352);
            let t16 = f64x8::splat(1.0) / t15;
            let t20 = (simd::ln(t4 * t10 * t16 / f64x8::splat(4.0)));
            let t21 = f64x8::splat(0.0310907) * t20;
            let t22 = t13 + f64x8::splat(3.72744);
            let t25 = (simd::atan(f64x8::splat(6.15199081975908) / t22));
            let t26 = f64x8::splat(0.038783294878113016) * t25;
            let t27 = t13 / f64x8::splat(2.0);
            let t28 = t27 + f64x8::splat(0.10498);
            let t29 = t28 * t28;
            let t31 = (simd::ln(t29 * t16));
            let t32 = f64x8::splat(0.0009690227711544374) * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = f64x8::splat(1.0) / t33;
            let t36 = t12 + f64x8::splat(0.565535) * t13 + f64x8::splat(13.0045);
            let t37 = f64x8::splat(1.0) / t36;
            let t41 = (simd::ln(t4 * t10 * t37 / f64x8::splat(4.0)));
            let t42 = t13 + f64x8::splat(1.13107);
            let t45 = (simd::atan(f64x8::splat(7.123108917818118) / t42));
            let t47 = t27 + f64x8::splat(0.0047584);
            let t48 = t47 * t47;
            let t50 = (simd::ln(t48 * t37));
            let t53 = t34 * (t41 + f64x8::splat(0.31770800474394145) * t45 + f64x8::splat(0.00041403379428206277) * t50);
            let t54 = v_rho0 - v_rho1;
            let t55 = f64x8::splat(1.0) / t7;
            let t56 = t54 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t62 = t61 * t57;
            let t63 = ((t58).select(t60, t62));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t67 = t66 * t64;
            let t68 = ((t65).select(t60, t67));
            let t69 = t63 + t68 - f64x8::splat(2.0);
            let t70 = t53 * t69;
            let t71 = f64x8::splat(M_CBRT2);
            let t72 = t71 - f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t72 / f64x8::splat(2.0);
            let t75 = t54 * t54;
            let t76 = t75 * t75;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = f64x8::splat(1.0) / t78;
            let t83 = f64x8::splat(9.0) * t72;
            let t84 = t74 * (-t76 * t79 + f64x8::splat(1.0)) * t83;
            let t86 = t70 * t84 / f64x8::splat(24.0);
            let t88 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = (simd::ln(t4 * t10 * t89 / f64x8::splat(4.0)));
            let t95 = t13 + f64x8::splat(7.06042);
            let t98 = (simd::atan(f64x8::splat(4.730926909560113) / t95));
            let t100 = t27 + f64x8::splat(0.325);
            let t101 = t100 * t100;
            let t103 = (simd::ln(t101 * t89));
            let t105 = f64x8::splat(0.01554535) * t93 + f64x8::splat(0.05249139316978094) * t98 + f64x8::splat(0.0022478670955426118) * t103 - t21 - t26 - t32;
            let t106 = t105 * t69;
            let t107 = t74 * t76;
            let t108 = t107 * t79;
            let t109 = t106 * t108;
            let t110 = t59 * t59;
            let t111 = t61 * t61;
            let t112 = ((t58).select(t110, t111));
            let t113 = t66 * t66;
            let t114 = ((t65).select(t110, t113));
            let t116 = t112 / f64x8::splat(2.0) + t114 / f64x8::splat(2.0);
            let t117 = t116 * t116;
            let t118 = t117 * t116;
            let t119 = param_gamma * t118;
            let t121 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t123 = f64x8::splat(1.0) / t8 / t77;
            let t124 = t121 * t123;
            let t126 = f64x8::splat(1.0) / t117;
            let t127 = t1 * t1;
            let t129 = f64x8::splat(1.0) / t3;
            let t130 = t129 * t5;
            let t131 = t126 * t127 * t130;
            let t134 = param_BB * param_beta;
            let t135 = f64x8::splat(1.0) / param_gamma;
            let t137 = (t21 + t26 + t32 - t86 + t109) * t135;
            let t138 = f64x8::splat(1.0) / t118;
            let t140 = (simd::exp(-t137 * t138));
            let t141 = t140 - f64x8::splat(1.0);
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t135 * t142;
            let t144 = t121 * t121;
            let t146 = t134 * t143 * t144;
            let t147 = t8 * t8;
            let t149 = f64x8::splat(1.0) / t147 / t78;
            let t150 = t71 * t71;
            let t151 = t149 * t150;
            let t152 = t117 * t117;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t155 = t3 * t3;
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t1 * t156;
            let t158 = t157 * t6;
            let t159 = t154 * t158;
            let t162 = t124 * t71 * t131 / f64x8::splat(96.0) + t146 * t159 / f64x8::splat(3072.0);
            let t163 = param_beta * t162;
            let t164 = param_beta * t135;
            let t167 = t164 * t142 * t162 + f64x8::splat(1.0);
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t135 * t168;
            let t171 = t163 * t169 + f64x8::splat(1.0);
            let t172 = (simd::ln(t171));
            let t173 = t119 * t172;
            let tzk0 = t21 + t26 + t32 - t86 + t109 + t173;
            acc_zk = tzk0;
            let t175 = f64x8::splat(1.0) / t8 / t7;
            let t176 = t6 * t175;
            let t180 = t4 * t6;
            let t181 = t15 * t15;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t9 * t182;
            let t184 = t4 * t176;
            let t185 = t184 / f64x8::splat(12.0);
            let t186 = f64x8::splat(1.0) / t13;
            let t187 = t186 * t1;
            let t188 = t3 * t6;
            let t190 = t187 * t188 * t175;
            let t192 = -t185 - f64x8::splat(0.31062) * t190;
            let t198 = (-t4 * t176 * t16 / f64x8::splat(12.0) - t180 * t183 * t192 / f64x8::splat(4.0)) * t127 * t129;
            let t199 = t5 * t8;
            let t200 = t199 * t15;
            let t201 = t198 * t200;
            let t202 = f64x8::splat(0.010363566666666667) * t201;
            let t203 = t22 * t22;
            let t204 = f64x8::splat(1.0) / t203;
            let t206 = t204 * t186 * t1;
            let t208 = f64x8::splat(37.8469910464) * t204 + f64x8::splat(1.0);
            let t209 = f64x8::splat(1.0) / t208;
            let t212 = t206 * t188 * t175 * t209;
            let t213 = f64x8::splat(0.03976574567502677) * t212;
            let t214 = t28 * t16;
            let t215 = t214 * t186;
            let t218 = t29 * t182;
            let t220 = -t215 * t184 / f64x8::splat(6.0) - t218 * t192;
            let t221 = f64x8::splat(1.0) / t29;
            let t222 = t220 * t221;
            let t223 = t222 * t15;
            let t224 = f64x8::splat(0.0009690227711544374) * t223;
            let t228 = t36 * t36;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t9 * t229;
            let t232 = -t185 - f64x8::splat(0.09425583333333333) * t190;
            let t238 = (-t4 * t176 * t37 / f64x8::splat(12.0) - t180 * t230 * t232 / f64x8::splat(4.0)) * t127 * t129;
            let t239 = t199 * t36;
            let t242 = t42 * t42;
            let t243 = f64x8::splat(1.0) / t242;
            let t245 = t243 * t186 * t1;
            let t247 = f64x8::splat(50.7386806551) * t243 + f64x8::splat(1.0);
            let t248 = f64x8::splat(1.0) / t247;
            let t253 = t47 * t37;
            let t254 = t253 * t186;
            let t257 = t48 * t229;
            let t259 = -t254 * t184 / f64x8::splat(6.0) - t257 * t232;
            let t260 = f64x8::splat(1.0) / t48;
            let t261 = t259 * t260;
            let t265 = t34 * (t238 * t239 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t245 * t188 * t175 * t248 + f64x8::splat(0.00041403379428206277) * t261 * t36);
            let t266 = t265 * t69;
            let t267 = t266 * t84;
            let t268 = t267 / f64x8::splat(24.0);
            let t269 = f64x8::splat(1.0) / t77;
            let t270 = t54 * t269;
            let t271 = t55 - t270;
            let t274 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t271));
            let t275 = -t271;
            let t278 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t275));
            let t279 = t274 + t278;
            let t280 = t53 * t279;
            let t281 = t280 * t84;
            let t282 = t281 / f64x8::splat(24.0);
            let t283 = t75 * t54;
            let t284 = t283 * t79;
            let t285 = t78 * t7;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t76 * t286;
            let t291 = t74 * (-f64x8::splat(4.0) * t284 + f64x8::splat(4.0) * t287) * t83;
            let t292 = t70 * t291;
            let t293 = t292 / f64x8::splat(24.0);
            let t297 = t88 * t88;
            let t298 = f64x8::splat(1.0) / t297;
            let t299 = t9 * t298;
            let t301 = -t185 - f64x8::splat(0.5883683333333334) * t190;
            let t307 = (-t4 * t176 * t89 / f64x8::splat(12.0) - t180 * t299 * t301 / f64x8::splat(4.0)) * t127 * t129;
            let t308 = t199 * t88;
            let t311 = t95 * t95;
            let t312 = f64x8::splat(1.0) / t311;
            let t314 = t312 * t186 * t1;
            let t316 = f64x8::splat(22.3816694236) * t312 + f64x8::splat(1.0);
            let t317 = f64x8::splat(1.0) / t316;
            let t322 = t100 * t89;
            let t323 = t322 * t186;
            let t326 = t101 * t298;
            let t328 = -t323 * t184 / f64x8::splat(6.0) - t326 * t301;
            let t329 = f64x8::splat(1.0) / t101;
            let t330 = t328 * t329;
            let t333 = f64x8::splat(0.005181783333333334) * t307 * t308 + f64x8::splat(0.041388824077869424) * t314 * t188 * t175 * t317 + f64x8::splat(0.0022478670955426118) * t330 * t88 - t202 - t213 - t224;
            let t334 = t333 * t69;
            let t335 = t334 * t108;
            let t336 = t105 * t279;
            let t337 = t336 * t108;
            let t338 = t74 * t283;
            let t339 = t338 * t79;
            let t340 = t106 * t339;
            let t341 = f64x8::splat(4.0) * t340;
            let t342 = t107 * t286;
            let t343 = t106 * t342;
            let t344 = f64x8::splat(4.0) * t343;
            let t345 = param_gamma * t117;
            let t346 = f64x8::splat(1.0) / t61;
            let t349 = ((t58).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t346 * t271));
            let t350 = f64x8::splat(1.0) / t66;
            let t353 = ((t65).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t275));
            let t355 = t349 / f64x8::splat(2.0) + t353 / f64x8::splat(2.0);
            let t356 = t172 * t355;
            let t357 = t345 * t356;
            let t358 = f64x8::splat(3.0) * t357;
            let t359 = t77 * t7;
            let t361 = f64x8::splat(1.0) / t8 / t359;
            let t362 = t121 * t361;
            let t365 = f64x8::splat(7.0) / f64x8::splat(288.0) * t362 * t71 * t131;
            let t366 = t71 * t138;
            let t367 = t124 * t366;
            let t368 = t127 * t129;
            let t369 = t5 * t355;
            let t370 = t368 * t369;
            let t373 = t134 * t135;
            let t374 = t141 * t141;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t375 * t144;
            let t378 = t373 * t376 * t149;
            let t379 = t150 * t153;
            let t380 = t379 * t1;
            let t381 = t156 * t6;
            let t383 = (t202 + t213 + t224 - t268 - t282 - t293 + t335 + t337 + t341 - t344) * t135;
            let t385 = t153 * t355;
            let t388 = f64x8::splat(3.0) * t137 * t385 - t383 * t138;
            let t389 = t388 * t140;
            let t390 = t381 * t389;
            let t391 = t380 * t390;
            let t395 = f64x8::splat(1.0) / t147 / t285;
            let t396 = t395 * t150;
            let t397 = t396 * t153;
            let t398 = t397 * t158;
            let t400 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t146 * t398;
            let t401 = t142 * t144;
            let t403 = t373 * t401 * t149;
            let t405 = f64x8::splat(1.0) / t152 / t116;
            let t406 = t150 * t405;
            let t407 = t406 * t1;
            let t409 = t407 * t381 * t355;
            let t412 = -t365 - t367 * t370 / f64x8::splat(48.0) - t378 * t391 / f64x8::splat(3072.0) - t400 - t403 * t409 / f64x8::splat(768.0);
            let t413 = param_beta * t412;
            let t415 = t167 * t167;
            let t416 = f64x8::splat(1.0) / t415;
            let t417 = t135 * t416;
            let t418 = t164 * t375;
            let t419 = t162 * t388;
            let t424 = -t418 * t419 * t140 + t164 * t142 * t412;
            let t425 = t417 * t424;
            let t427 = -t163 * t425 + t413 * t169;
            let t428 = f64x8::splat(1.0) / t171;
            let t429 = t427 * t428;
            let t430 = t119 * t429;
            let t431 = t202 + t213 + t224 - t268 - t282 - t293 + t335 + t337 + t341 - t344 + t358 + t430;
            let tvrho0 = t7 * t431 + t109 + t173 + t21 + t26 + t32 - t86;
            acc_vrho_0 = tvrho0;
            let t433 = -t55 - t270;
            let t436 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t433));
            let t437 = -t433;
            let t440 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t437));
            let t441 = t436 + t440;
            let t442 = t53 * t441;
            let t443 = t442 * t84;
            let t444 = t443 / f64x8::splat(24.0);
            let t448 = t74 * (f64x8::splat(4.0) * t284 + f64x8::splat(4.0) * t287) * t83;
            let t449 = t70 * t448;
            let t450 = t449 / f64x8::splat(24.0);
            let t451 = t105 * t441;
            let t452 = t451 * t108;
            let t455 = ((t58).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t346 * t433));
            let t458 = ((t65).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t437));
            let t460 = t455 / f64x8::splat(2.0) + t458 / f64x8::splat(2.0);
            let t461 = t172 * t460;
            let t462 = t345 * t461;
            let t463 = f64x8::splat(3.0) * t462;
            let t464 = t5 * t460;
            let t465 = t368 * t464;
            let t469 = (t202 + t213 + t224 - t268 - t444 - t450 + t335 + t452 - t341 - t344) * t135;
            let t471 = t153 * t460;
            let t474 = f64x8::splat(3.0) * t137 * t471 - t469 * t138;
            let t475 = t474 * t140;
            let t476 = t381 * t475;
            let t477 = t380 * t476;
            let t481 = t407 * t381 * t460;
            let t484 = -t365 - t367 * t465 / f64x8::splat(48.0) - t378 * t477 / f64x8::splat(3072.0) - t400 - t403 * t481 / f64x8::splat(768.0);
            let t485 = param_beta * t484;
            let t487 = t162 * t474;
            let t492 = -t418 * t487 * t140 + t164 * t142 * t484;
            let t493 = t417 * t492;
            let t495 = -t163 * t493 + t485 * t169;
            let t496 = t495 * t428;
            let t497 = t119 * t496;
            let t498 = t202 + t213 + t224 - t268 - t444 - t450 + t335 + t452 - t341 - t344 + t463 + t497;
            let tvrho1 = t7 * t498 + t109 + t173 + t21 + t26 + t32 - t86;
            acc_vrho_1 = tvrho1;
            let t500 = t7 * param_gamma;
            let t501 = t123 * t71;
            let t503 = t368 * t5;
            let t504 = t501 * t126 * t503;
            let t507 = t134 * t143 * t121;
            let t508 = t507 * t159;
            let t510 = t504 / f64x8::splat(96.0) + t508 / f64x8::splat(1536.0);
            let t511 = param_beta * t510;
            let t513 = param_beta * param_beta;
            let t514 = t513 * t162;
            let t515 = param_gamma * param_gamma;
            let t516 = f64x8::splat(1.0) / t515;
            let t517 = t514 * t516;
            let t518 = t416 * t142;
            let t519 = t518 * t510;
            let t521 = t511 * t169 - t517 * t519;
            let tvsigma0 = t500 * t118 * t521 * t428;
            acc_vsigma_0 = tvsigma0;
            let t526 = t504 / f64x8::splat(48.0) + t508 / f64x8::splat(768.0);
            let t527 = param_beta * t526;
            let t529 = t518 * t526;
            let t531 = t527 * t169 - t517 * t529;
            let t532 = t118 * t531;
            let tvsigma1 = t500 * t532 * t428;
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
