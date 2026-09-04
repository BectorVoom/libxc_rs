//! LDA_C_W20 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_w20.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_w20_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = (simd::ln(f64x8::splat(2.0)));
            let t2 = f64x8::splat(1.0) - t1;
            let t3 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = t1 / f64x8::splat(6.0);
            let t8 = f64x8::splat(1.0) / t2;
            let t12 = (simd::exp(-f64x8::splat(2.0) * (-f64x8::splat(0.16244537117517982) + t6) * t8 * t3));
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t16 = (simd::cbrt(t15));
            let t17 = t16 * t16;
            let t18 = t14 * t17;
            let t19 = f64x8::splat(M_CBRT4);
            let t20 = (simd::cbrt(v_rho));
            let t21 = t20 * t20;
            let t22 = f64x8::splat(1.0) / t21;
            let t24 = t18 * t19 * t22;
            let t26 = (simd::exp(-t24 / f64x8::splat(40000.0)));
            let t27 = f64x8::splat(1.0) - t26;
            let t28 = f64x8::splat(M_CBRTPI);
            let t29 = t28 * t28;
            let t31 = (simd::cbrt(f64x8::splat(9.0)));
            let t32 = f64x8::splat(1.0) / t29 * t31;
            let t33 = t19 * t19;
            let t39 = t12 / f64x8::splat(2.0);
            let t40 = (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t32 * t33) * t8 * t3 + t39;
            let t44 = (-f64x8::splat(2.0) * t27 * t40 + t12) * t14;
            let t45 = f64x8::splat(1.0) / t16;
            let t46 = t45 * t19;
            let t47 = t46 * t20;
            let t50 = t27 * t8;
            let t51 = ((f64x8::splat(4.0)).sqrt());
            let t52 = t13 * t16;
            let t53 = f64x8::splat(1.0) / t20;
            let t55 = t52 * t33 * t53;
            let t56 = ((t55).sqrt());
            let t58 = f64x8::splat(1.0) / t56 / t55;
            let t60 = t50 * t51 * t58;
            let t62 = t31 * t31;
            let t63 = t62 * t19;
            let t64 = t29 * t3;
            let t68 = -f64x8::splat(3.0) / f64x8::splat(40.0) * t63 * t64 * t8 + t39;
            let t72 = (-f64x8::splat(2.0) * t27 * t68 + t12) * t13;
            let t73 = f64x8::splat(1.0) / t17;
            let t74 = t73 * t33;
            let t75 = t74 * t21;
            let t78 = f64x8::splat(1.0) + t44 * t47 / f64x8::splat(3.0) - f64x8::splat(118.43525281307231) * t60 + t72 * t75 / f64x8::splat(3.0);
            let t79 = (simd::ln(t78));
            let t81 = t5 * t79 / f64x8::splat(2.0);
            let t82 = t52 * t33;
            let t83 = t53 * t26;
            let t84 = ((f64x8::splat(4.0)).sqrt().sqrt());
            let t85 = t84 * t84;
            let t86 = t85 * t84;
            let t87 = ((t55).sqrt().sqrt());
            let t91 = t26 + f64x8::splat(5.0) / f64x8::splat(8.0) * t86 * t87 * t55;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t3 * f64x8::splat(M_PI);
            let t95 = f64x8::splat(1.0) / t28 / t93;
            let t97 = f64x8::splat(12.0) * t1;
            let t98 = f64x8::splat(7.0) / f64x8::splat(6.0) * t3 - t97 - f64x8::splat(1.0);
            let t99 = t95 * t98;
            let t100 = t14 * t45;
            let t104 = f64x8::splat(1.0) + t100 * t19 * t20 / f64x8::splat(3.0);
            let t105 = (simd::ln(t104));
            let t109 = -t63 * t99 * t105 / f64x8::splat(36.0) - f64x8::splat(0.01);
            let t110 = t92 * t109;
            let t113 = t82 * t83 * t110 / f64x8::splat(4.0);
            let t118 = (simd::exp(-f64x8::splat(4.0) * (-f64x8::splat(0.1412623711751798) + t6) * t8 * t3));
            let t119 = f64x8::splat(M_CBRT2);
            let t127 = t118 / f64x8::splat(2.0);
            let t128 = f64x8::splat(2.0) * (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t32 * t33 * t119) * t8 * t3 + t127;
            let t132 = (-f64x8::splat(2.0) * t27 * t128 + t118) * t14;
            let t136 = t119 * t119;
            let t141 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t63 * t64 * t136 * t8 + t127;
            let t145 = (-f64x8::splat(2.0) * t27 * t141 + t118) * t13;
            let t148 = f64x8::splat(1.0) + t132 * t47 / f64x8::splat(3.0) - f64x8::splat(236.87050562614462) * t60 + t145 * t75 / f64x8::splat(3.0);
            let t149 = (simd::ln(t148));
            let t154 = t136 * t62;
            let t156 = f64x8::splat(13.0) / f64x8::splat(12.0) * t3 - t97 + f64x8::splat(1.0) / f64x8::splat(2.0);
            let t157 = t95 * t156;
            let t159 = t154 * t157 * t105;
            let t164 = (simd::cbrt(zeta_threshold));
            let t166 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t164 * zeta_threshold, f64x8::splat(1.0)));
            let t168 = f64x8::splat(2.0) * t166 - f64x8::splat(2.0);
            let t172 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t119 - f64x8::splat(2.0));
            let t173 = (-t5 * t149 / f64x8::splat(4.0) - t52 * t83 * t92 * t159 / f64x8::splat(144.0) + t81 - t113) * t168 * t172;
            let tzk0 = -t81 + t113 + t173;
            acc_zk = tzk0;
            let t175 = f64x8::splat(1.0) / t20 / v_rho;
            let t176 = t175 * t26;
            let t180 = t46 * t22;
            let t183 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t184 = t183 * t183;
            let t185 = t184 * t184;
            let t186 = t185 * t183;
            let t187 = t18 * t186;
            let t189 = f64x8::splat(1.0) / t21 / v_rho;
            let t190 = t189 * t26;
            let t191 = t8 * t58;
            let t193 = t187 * t190 * t191;
            let t195 = t50 * t183;
            let t196 = f64x8::splat(4.0) * t24;
            let t198 = f64x8::splat(1.0) / t56 / t196;
            let t199 = t198 * t13;
            let t202 = t195 * t199 * t16 * t175;
            let t204 = f64x8::splat(1.0) / v_rho;
            let t205 = t204 * t26;
            let t208 = t74 * t53;
            let t211 = t82 * t176 * t40 / f64x8::splat(30000.0) + t44 * t180 / f64x8::splat(9.0) + f64x8::splat(0.0019739208802178718) * t193 - f64x8::splat(236.87050562614462) * t202 + t205 * t68 / f64x8::splat(7500.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t72 * t208;
            let t212 = f64x8::splat(1.0) / t78;
            let t214 = t5 * t211 * t212;
            let t215 = t214 / f64x8::splat(2.0);
            let t217 = t82 * t176 * t110;
            let t218 = t217 / f64x8::splat(12.0);
            let t219 = v_rho * v_rho;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t15 * t220;
            let t222 = t26 * t92;
            let t223 = t222 * t109;
            let t224 = t221 * t223;
            let t225 = t224 / f64x8::splat(20000.0);
            let t226 = t91 * t91;
            let t227 = f64x8::splat(1.0) / t226;
            let t228 = t26 * t227;
            let t229 = t19 * t189;
            let t233 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t234 = t233 * t233;
            let t235 = t234 * t234;
            let t236 = t235 * t233;
            let t237 = t236 * t87;
            let t238 = t52 * t175;
            let t241 = t18 * t229 * t26 / f64x8::splat(60000.0) - f64x8::splat(25.0) / f64x8::splat(24.0) * t237 * t238;
            let t242 = t109 * t241;
            let t243 = t228 * t242;
            let t244 = t55 * t243;
            let t245 = t244 / f64x8::splat(4.0);
            let t246 = t19 * t204;
            let t248 = t62 * t95;
            let t249 = f64x8::splat(1.0) / t104;
            let t250 = t98 * t249;
            let t251 = t248 * t250;
            let t252 = t246 * t222 * t251;
            let t253 = t252 / f64x8::splat(108.0);
            let t265 = t82 * t176 * t128 / f64x8::splat(30000.0) + t132 * t180 / f64x8::splat(9.0) + f64x8::splat(0.0039478417604357436) * t193 - f64x8::splat(473.74101125228924) * t202 + t205 * t141 / f64x8::splat(7500.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t145 * t208;
            let t266 = f64x8::splat(1.0) / t148;
            let t274 = t3 * t3;
            let t276 = f64x8::splat(1.0) / t28 / t274;
            let t277 = t276 * t220;
            let t278 = t19 * t26;
            let t280 = t92 * t136;
            let t281 = t62 * t156;
            let t282 = t281 * t105;
            let t283 = t280 * t282;
            let t287 = t52 * t83 * t227;
            let t288 = t154 * t95;
            let t289 = t156 * t105;
            let t290 = t289 * t241;
            let t291 = t288 * t290;
            let t297 = t248 * t156 * t19 * t249;
            let t302 = (-t5 * t265 * t266 / f64x8::splat(4.0) + t52 * t176 * t92 * t159 / f64x8::splat(432.0) - t277 * t278 * t283 / f64x8::splat(2880000.0) + t287 * t291 / f64x8::splat(144.0) - t205 * t280 * t297 / f64x8::splat(432.0) + t215 + t218 - t225 + t245 + t253) * t168 * t172;
            let tvrho0 = -t81 + t113 + t173 + v_rho * (-t215 - t218 + t225 - t245 - t253 + t302);
            acc_vrho = tvrho0;
            let t311 = f64x8::splat(1.0) / t20 / t219;
            let t312 = t311 * t26;
            let t316 = t219 * v_rho;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t15 * t317;
            let t319 = t26 * t40;
            let t322 = t46 * t189;
            let t326 = f64x8::splat(1.0) / t21 / t219;
            let t327 = t326 * t26;
            let t329 = t187 * t327 * t191;
            let t331 = t16 * t15;
            let t332 = t13 * t331;
            let t333 = t332 * t183;
            let t335 = f64x8::splat(1.0) / t20 / t316;
            let t336 = t335 * t26;
            let t338 = t333 * t336 * t191;
            let t340 = t15 * t51;
            let t342 = t26 * t8;
            let t343 = t342 * t198;
            let t344 = t340 * t317 * t343;
            let t346 = t50 * t186;
            let t350 = f64x8::splat(1.0) / t56 / t15 / t204 / f64x8::splat(48.0);
            let t351 = t350 * t14;
            let t354 = t346 * t351 * t17 * t326;
            let t358 = t195 * t199 * t16 * t311;
            let t360 = t220 * t26;
            let t364 = t326 * t14 * t17;
            let t365 = t278 * t68;
            let t368 = t74 * t175;
            let t371 = -t82 * t312 * t40 / f64x8::splat(30000.0) + t318 * t319 / f64x8::splat(150000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t44 * t322 - f64x8::splat(0.003289868133696453) * t329 + f64x8::splat(3.9478417604357434e-07) * t338 + f64x8::splat(0.02368705056261446) * t344 - f64x8::splat(197.39208802178717) * t354 + f64x8::splat(315.82734083485946) * t358 - t360 * t68 / f64x8::splat(22500.0) + t364 * t365 / f64x8::splat(450000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t72 * t368;
            let t373 = t5 * t371 * t212;
            let t374 = t373 / f64x8::splat(2.0);
            let t375 = t211 * t211;
            let t376 = t78 * t78;
            let t377 = f64x8::splat(1.0) / t376;
            let t379 = t5 * t375 * t377;
            let t380 = t379 / f64x8::splat(2.0);
            let t382 = t82 * t312 * t110;
            let t383 = t382 / f64x8::splat(9.0);
            let t384 = t318 * t223;
            let t385 = f64x8::splat(7.0) / f64x8::splat(60000.0) * t384;
            let t386 = t33 * t175;
            let t387 = t52 * t386;
            let t388 = t387 * t243;
            let t389 = t388 / f64x8::splat(6.0);
            let t390 = t19 * t220;
            let t392 = t390 * t222 * t251;
            let t393 = t392 / f64x8::splat(81.0);
            let t395 = f64x8::splat(1.0) / t21 / t316;
            let t397 = t15 * t395 * t18;
            let t398 = t278 * t110;
            let t399 = t397 * t398;
            let t400 = t399 / f64x8::splat(1200000000.0);
            let t401 = t221 * t26;
            let t402 = t227 * t109;
            let t403 = t402 * t241;
            let t404 = t401 * t403;
            let t405 = t404 / f64x8::splat(10000.0);
            let t406 = t276 * t326;
            let t407 = t222 * t62;
            let t409 = t33 * t98;
            let t410 = t100 * t249;
            let t411 = t409 * t410;
            let t412 = t406 * t407 * t411;
            let t413 = t412 / f64x8::splat(6480000.0);
            let t415 = f64x8::splat(1.0) / t226 / t91;
            let t416 = t26 * t415;
            let t417 = t241 * t241;
            let t418 = t109 * t417;
            let t419 = t416 * t418;
            let t420 = t55 * t419;
            let t421 = t420 / f64x8::splat(2.0);
            let t422 = t246 * t228;
            let t423 = t250 * t241;
            let t424 = t248 * t423;
            let t425 = t422 * t424;
            let t426 = t425 / f64x8::splat(54.0);
            let t431 = t33 * t335;
            let t435 = t87 * t87;
            let t436 = t435 * t87;
            let t438 = t233 / t436;
            let t444 = -t18 * t19 * t326 * t26 / f64x8::splat(36000.0) + t332 * t431 * t26 / f64x8::splat(1200000000.0) + f64x8::splat(25.0) / f64x8::splat(72.0) * t438 * t364 + f64x8::splat(25.0) / f64x8::splat(18.0) * t237 * t52 * t311;
            let t446 = t228 * t109 * t444;
            let t447 = t55 * t446;
            let t448 = t447 / f64x8::splat(4.0);
            let t449 = t33 * t326;
            let t450 = t18 * t26;
            let t451 = t449 * t450;
            let t452 = t92 * t62;
            let t454 = t452 * t99 * t249;
            let t455 = t451 * t454;
            let t456 = t455 / f64x8::splat(6480000.0);
            let t457 = t33 * t189;
            let t459 = t104 * t104;
            let t460 = f64x8::splat(1.0) / t459;
            let t462 = t460 * t14 * t45;
            let t463 = t99 * t462;
            let t464 = t457 * t407 * t463;
            let t465 = t464 / f64x8::splat(972.0);
            let t469 = t26 * t128;
            let t481 = t278 * t141;
            let t486 = -t82 * t312 * t128 / f64x8::splat(30000.0) + t318 * t469 / f64x8::splat(150000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t132 * t322 - f64x8::splat(0.006579736267392906) * t329 + f64x8::splat(7.895683520871487e-07) * t338 + f64x8::splat(0.04737410112522892) * t344 - f64x8::splat(394.78417604357435) * t354 + f64x8::splat(631.6546816697189) * t358 - t360 * t141 / f64x8::splat(22500.0) + t364 * t481 / f64x8::splat(450000000.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t145 * t368;
            let t490 = t265 * t265;
            let t491 = t148 * t148;
            let t492 = f64x8::splat(1.0) / t491;
            let t499 = t276 * t317;
            let t503 = t385 - t393 - t400 - t421 + t448 - t389 - t5 * t486 * t266 / f64x8::splat(4.0) + t5 * t490 * t492 / f64x8::splat(4.0) + t374 - t380 + t405 + t360 * t280 * t297 / f64x8::splat(324.0) - t426 + f64x8::splat(7.0) / f64x8::splat(8640000.0) * t499 * t278 * t283;
            let t504 = t280 * t62;
            let t505 = t157 * t249;
            let t506 = t504 * t505;
            let t511 = t157 * t33 * t462;
            let t515 = t52 * t176 * t227;
            let t519 = t52 * t83 * t415;
            let t520 = t289 * t417;
            let t521 = t288 * t520;
            let t524 = t276 * t395;
            let t526 = t33 * t14 * t17;
            let t528 = t222 * t136;
            let t529 = t528 * t282;
            let t532 = t33 * t26;
            let t533 = t532 * t92;
            let t535 = t154 * t156;
            let t536 = t535 * t410;
            let t539 = t289 * t444;
            let t540 = t288 * t539;
            let t547 = t278 * t227;
            let t548 = t277 * t547;
            let t549 = t154 * t290;
            let t553 = t227 * t136 * t62;
            let t554 = t205 * t553;
            let t555 = t19 * t249;
            let t557 = t157 * t555 * t241;
            let t560 = -t383 - t451 * t506 / f64x8::splat(25920000.0) + t190 * t504 * t511 / f64x8::splat(3888.0) - t515 * t291 / f64x8::splat(216.0) - t519 * t521 / f64x8::splat(72.0) - t524 * t526 * t529 / f64x8::splat(172800000000.0) - t406 * t533 * t536 / f64x8::splat(25920000.0) + t287 * t540 / f64x8::splat(144.0) - t52 * t312 * t92 * t159 / f64x8::splat(324.0) + t413 + t456 - t465 + t548 * t549 / f64x8::splat(1440000.0) + t554 * t557 / f64x8::splat(216.0);
            let t563 = (t503 + t560) * t168 * t172;
            let t564 = -t374 + t380 + t383 - t385 + t389 + t393 + t400 - t405 - t413 + t421 + t426 - t448 - t456 + t465 + t563;
            let tv2rho20 = -t214 - t217 / f64x8::splat(6.0) + t224 / f64x8::splat(10000.0) - t244 / f64x8::splat(2.0) - t252 / f64x8::splat(54.0) + f64x8::splat(2.0) * t302 + v_rho * t564;
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
