//! LDA_C_PW_ERF vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw_erf.c`
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
pub fn lda_c_pw_erf_vxc_pol(
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
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = t5 * t23;
            let t25 = t21 * t24;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.03109) * t62 * t71 + t33 - f64x8::splat(0.019751789702565206) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.019751789702565206) * t60 * t85;
            let t92 = t47 * t47;
            let t93 = t52 * t52;
            let t95 = t92 / f64x8::splat(2.0) + t93 / f64x8::splat(2.0);
            let t96 = t95 * t95;
            let t97 = t96 * t95;
            let t98 = (simd::ln(f64x8::splat(2.0)));
            let t99 = t98 - f64x8::splat(1.0);
            let t100 = f64x8::splat(2.0) * t99;
            let t101 = t97 * t100;
            let t102 = param_hyb_omega_0 * t14;
            let t103 = f64x8::splat(1.0) / t95;
            let t105 = f64x8::splat(2.923025) * t102 * t103;
            let t107 = (simd::cbrt(f64x8::splat(9.0)));
            let t108 = t107 * t107;
            let t116 = param_hyb_omega_0 * param_hyb_omega_0;
            let t117 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t5 * t108 * t3 / t99 / f64x8::splat(12.0)) * t116;
            let t118 = t117 * t1;
            let t119 = t3 * t6;
            let t120 = f64x8::splat(1.0) / t96;
            let t125 = t116 * param_hyb_omega_0;
            let t126 = t14 * t11;
            let t127 = t125 * t126;
            let t128 = f64x8::splat(1.0) / t97;
            let t131 = f64x8::splat(1.0) + t105 + t118 * t119 * t9 * t120 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t127 * t128;
            let t132 = t116 * t1;
            let t133 = t132 * t3;
            let t137 = f64x8::splat(1.0) + t105 + f64x8::splat(0.8621275) * t133 * t10 * t120;
            let t138 = f64x8::splat(1.0) / t137;
            let t140 = (simd::ln(t131 * t138));
            let t141 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t140 * t142;
            let t145 = f64x8::splat(1.0) / t37;
            let t147 = -t35 * t145 + f64x8::splat(1.0);
            let t148 = t41 * t147;
            let t152 = t3 * t2;
            let t153 = t1 * t152;
            let t155 = f64x8::splat(1.0) / t8 / t7;
            let t156 = t6 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.005175) * t11 + f64x8::splat(0.0204825) * t25 - f64x8::splat(0.0030486129349252553) * t41 + f64x8::splat(0.0003485625) * t153 * t156;
            let t161 = (simd::exp(-f64x8::splat(0.1881) * t11));
            let t162 = t159 * t161;
            let t163 = f64x8::splat(M_SQRT2);
            let t164 = t162 * t163;
            let t168 = t19 * t20 * t142;
            let t169 = t168 * t5;
            let t171 = f64x8::splat(1.0) / t22 / t7;
            let t172 = t43 / f64x8::splat(2.0);
            let t173 = t172 * t172;
            let t174 = t4 * t6;
            let t175 = t9 * t56;
            let t176 = f64x8::splat(1.0) / t43;
            let t177 = (simd::cbrt(t176));
            let t179 = t174 * t175 * t177;
            let t181 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t179;
            let t182 = t173 * t181;
            let t183 = f64x8::splat(1.0) / t152;
            let t184 = t108 * t183;
            let t185 = t182 * t184;
            let t186 = t1 * t22;
            let t187 = t177 * t177;
            let t188 = f64x8::splat(1.0) / t187;
            let t190 = t21 * t5;
            let t191 = t56 * t56;
            let t192 = t23 * t191;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t179 + f64x8::splat(0.01) * t190 * t192 * t187;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t188 * t197;
            let t199 = t186 * t198;
            let t201 = f64x8::splat(2.0) / f64x8::splat(15.0) * t185 * t199;
            let t202 = t50 / f64x8::splat(2.0);
            let t203 = t202 * t202;
            let t204 = f64x8::splat(1.0) / t50;
            let t205 = (simd::cbrt(t204));
            let t207 = t174 * t175 * t205;
            let t209 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t207;
            let t210 = t203 * t209;
            let t211 = t210 * t184;
            let t212 = t205 * t205;
            let t213 = f64x8::splat(1.0) / t212;
            let t218 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t207 + f64x8::splat(0.01) * t190 * t192 * t212;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t213 * t219;
            let t221 = t186 * t220;
            let t223 = f64x8::splat(2.0) / f64x8::splat(15.0) * t211 * t221;
            let t225 = (simd::exp(-f64x8::splat(0.0775) * t11));
            let t226 = t147 * t225;
            let t229 = -f64x8::splat(1.2375) * t11 + t25 / f64x8::splat(4.0);
            let t230 = t229 * f64x8::splat(M_PI);
            let t231 = t230 * t7;
            let t234 = t201 + t223 + f64x8::splat(4.0) / f64x8::splat(3.0) * t226 * t231;
            let t242 = t162 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t245 = t5 * t171;
            let t247 = (simd::exp(-f64x8::splat(0.13675) * t11));
            let t248 = t147 * t247;
            let t251 = -f64x8::splat(0.097) * t11 + f64x8::splat(0.169) * t25;
            let t252 = t248 * t251;
            let t254 = t1 / t20;
            let t256 = t254 * t6 * t22;
            let t259 = t43 * t43;
            let t260 = t92 * t259;
            let t261 = t50 * t50;
            let t262 = t93 * t261;
            let t265 = (t260 / f64x8::splat(2.0) + t262 / f64x8::splat(2.0)) * t108;
            let t266 = t183 * t1;
            let t267 = t266 * t22;
            let t270 = t201 + t223 + t252 * t256 / f64x8::splat(3.0) - t265 * t267 / f64x8::splat(15.0);
            let t274 = -t33 + t89 + t91;
            let t279 = t116 * t116;
            let t281 = t168 * t245;
            let t283 = t161 * t163;
            let t285 = t283 * t279 * param_hyb_omega_0;
            let t286 = t147 * t159 * t285;
            let t289 = t171 * t147;
            let t296 = t279 * t116;
            let t299 = f64x8::splat(1.0) / t22 / t37;
            let t301 = t279 * t279;
            let t305 = t101 * t143 + (-f64x8::splat(0.031505407223141116) * t148 * t164 - f64x8::splat(0.005388405304614574) * t169 * t171 * t234 * t163) * t125 + (-f64x8::splat(0.0837628205355044) * t148 * t242 - f64x8::splat(0.011938374665504766) * t168 * t245 * t270 + f64x8::splat(0.42708890021612717) * t153 * t156 * t274) * t279 - f64x8::splat(0.01197423401025461) * t281 * t286 + (-f64x8::splat(0.031835665774679375) * t169 * t289 * t242 + f64x8::splat(0.05332506774217938) * t145 * t274) * t296 + f64x8::splat(0.020267214298646783) * t169 * t299 * t274 * t301;
            let t309 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t21 * t24 * t116;
            let t310 = t309 * t309;
            let t311 = t310 * t310;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t305 * t312;
            let tzk0 = -t33 + t89 + t91 - t313;
            acc_zk = tzk0;
            let t315 = t4 * t156 * t31;
            let t316 = f64x8::splat(0.0011073577833333333) * t315;
            let t317 = t27 * t27;
            let t318 = f64x8::splat(1.0) / t317;
            let t319 = t13 * t318;
            let t320 = f64x8::splat(1.0) / t14;
            let t321 = t320 * t1;
            let t322 = t119 * t155;
            let t323 = t321 * t322;
            let t325 = t4 * t156;
            let t327 = ((t11).sqrt());
            let t328 = t327 * t1;
            let t329 = t328 * t322;
            let t331 = t21 * t245;
            let t333 = -f64x8::splat(0.632975) * t323 - f64x8::splat(0.29896666666666666) * t325 - f64x8::splat(0.1023875) * t329 - f64x8::splat(0.08215666666666667) * t331;
            let t334 = f64x8::splat(1.0) / t30;
            let t335 = t333 * t334;
            let t336 = t319 * t335;
            let t337 = f64x8::splat(1.0) * t336;
            let t338 = t35 * t34;
            let t339 = t338 * t39;
            let t340 = t339 * t88;
            let t341 = f64x8::splat(4.0) * t340;
            let t342 = t38 * t7;
            let t343 = f64x8::splat(1.0) / t342;
            let t344 = t36 * t343;
            let t345 = t344 * t88;
            let t346 = f64x8::splat(4.0) * t345;
            let t347 = t34 * t145;
            let t348 = t41 - t347;
            let t351 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t348));
            let t352 = -t348;
            let t355 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t352));
            let t357 = (t351 + t355) * t59;
            let t358 = t357 * t87;
            let t359 = t40 * t358;
            let t363 = t67 * t67;
            let t364 = f64x8::splat(1.0) / t363;
            let t365 = t62 * t364;
            let t370 = -f64x8::splat(1.176575) * t323 - f64x8::splat(0.516475) * t325 - f64x8::splat(0.2103875) * t329 - f64x8::splat(0.104195) * t331;
            let t371 = f64x8::splat(1.0) / t70;
            let t372 = t370 * t371;
            let t378 = t80 * t80;
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = t75 * t379;
            let t385 = -f64x8::splat(0.8630833333333333) * t323 - f64x8::splat(0.301925) * t325 - f64x8::splat(0.05501625) * t329 - f64x8::splat(0.082785) * t331;
            let t386 = f64x8::splat(1.0) / t83;
            let t387 = t385 * t386;
            let t390 = f64x8::splat(0.0005323644333333333) * t4 * t156 * t71 + f64x8::splat(1.0) * t365 * t372 - t316 - t337 + f64x8::splat(0.0001831155503675316) * t4 * t156 * t84 + f64x8::splat(0.5848223397455204) * t380 * t387;
            let t391 = t60 * t390;
            let t392 = t40 * t391;
            let t393 = t357 * t85;
            let t394 = f64x8::splat(0.019751789702565206) * t393;
            let t395 = t60 * t1;
            let t397 = t119 * t155 * t84;
            let t398 = t395 * t397;
            let t399 = f64x8::splat(0.0001831155503675316) * t398;
            let t400 = t60 * t75;
            let t402 = t379 * t385 * t386;
            let t403 = t400 * t402;
            let t404 = f64x8::splat(0.5848223397455204) * t403;
            let t405 = t96 * t100;
            let t406 = f64x8::splat(1.0) / t47;
            let t408 = f64x8::splat(1.0) / t52;
            let t411 = t406 * t348 / f64x8::splat(3.0) + t408 * t352 / f64x8::splat(3.0);
            let t415 = param_hyb_omega_0 * t320;
            let t416 = t415 * t103;
            let t418 = f64x8::splat(0.48717083333333333) * t416 * t325;
            let t421 = f64x8::splat(2.923025) * t102 * t120 * t411;
            let t425 = t118 * t119 * t155 * t120 / f64x8::splat(12.0);
            let t426 = t117 * t4;
            let t427 = t128 * t411;
            let t428 = t10 * t427;
            let t431 = t125 * t14;
            let t432 = t431 * t128;
            let t434 = f64x8::splat(0.24484) * t432 * t325;
            let t435 = t96 * t96;
            let t436 = f64x8::splat(1.0) / t435;
            let t437 = t436 * t411;
            let t440 = -t418 - t421 - t425 - t426 * t428 / f64x8::splat(2.0) - t434 - f64x8::splat(1.46904) * t127 * t437;
            let t442 = t137 * t137;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = t131 * t443;
            let t447 = f64x8::splat(0.28737583333333333) * t133 * t156 * t120;
            let t450 = -t418 - t421 - t447 - f64x8::splat(1.724255) * t133 * t428;
            let t452 = t440 * t138 - t444 * t450;
            let t453 = t101 * t452;
            let t454 = f64x8::splat(1.0) / t131;
            let t455 = t454 * t137;
            let t456 = t455 * t142;
            let t458 = t145 * t147;
            let t460 = f64x8::splat(0.031505407223141116) * t458 * t164;
            let t461 = t37 * t7;
            let t462 = f64x8::splat(1.0) / t461;
            let t463 = t35 * t462;
            let t465 = -f64x8::splat(2.0) * t347 + f64x8::splat(2.0) * t463;
            let t466 = t41 * t465;
            let t473 = f64x8::splat(1.0) / t8 / t37;
            let t474 = t6 * t473;
            let t477 = -f64x8::splat(0.001725) * t325 - f64x8::splat(0.013655) * t331 + f64x8::splat(0.0030486129349252553) * t145 - f64x8::splat(0.00046475) * t153 * t474;
            let t478 = t477 * t161;
            let t479 = t478 * t163;
            let t481 = f64x8::splat(0.031505407223141116) * t148 * t479;
            let t482 = t473 * t147;
            let t483 = t159 * t1;
            let t485 = t119 * t283;
            let t487 = f64x8::splat(0.001975389032890948) * t482 * t483 * t485;
            let t491 = f64x8::splat(0.008980675507690957) * t169 * t299 * t234 * t163;
            let t492 = t172 * t181;
            let t493 = t492 * t184;
            let t494 = t348 / f64x8::splat(2.0);
            let t495 = t198 * t494;
            let t496 = t186 * t495;
            let t498 = f64x8::splat(4.0) / f64x8::splat(15.0) * t493 * t496;
            let t499 = t155 * t56;
            let t501 = t174 * t499 * t177;
            let t502 = f64x8::splat(0.0018891666666666666) * t501;
            let t503 = t56 * t188;
            let t504 = f64x8::splat(1.0) / t259;
            let t505 = t504 * t348;
            let t506 = t503 * t505;
            let t507 = t11 * t506;
            let t509 = t502 + f64x8::splat(0.0018891666666666666) * t507;
            let t510 = t173 * t509;
            let t511 = t510 * t184;
            let t513 = f64x8::splat(2.0) / f64x8::splat(15.0) * t511 * t199;
            let t514 = t1 * t9;
            let t515 = t514 * t198;
            let t517 = f64x8::splat(4.0) / f64x8::splat(45.0) * t185 * t515;
            let t518 = t184 * t1;
            let t519 = t182 * t518;
            let t521 = f64x8::splat(1.0) / t187 / t176;
            let t522 = t22 * t521;
            let t523 = t197 * t504;
            let t524 = t523 * t348;
            let t525 = t522 * t524;
            let t527 = f64x8::splat(4.0) / f64x8::splat(45.0) * t519 * t525;
            let t528 = t196 * t196;
            let t529 = f64x8::splat(1.0) / t528;
            let t530 = t188 * t529;
            let t531 = f64x8::splat(0.035991666666666665) * t501;
            let t533 = t171 * t191;
            let t536 = f64x8::splat(0.006666666666666667) * t190 * t533 * t187;
            let t537 = f64x8::splat(1.0) / t177;
            let t538 = t191 * t537;
            let t539 = t538 * t505;
            let t542 = -t531 - f64x8::splat(0.035991666666666665) * t507 - t536 - f64x8::splat(0.006666666666666667) * t25 * t539;
            let t543 = t530 * t542;
            let t544 = t186 * t543;
            let t546 = f64x8::splat(2.0) / f64x8::splat(15.0) * t185 * t544;
            let t547 = t202 * t209;
            let t548 = t547 * t184;
            let t549 = -t494;
            let t550 = t220 * t549;
            let t551 = t186 * t550;
            let t553 = f64x8::splat(4.0) / f64x8::splat(15.0) * t548 * t551;
            let t555 = t174 * t499 * t205;
            let t556 = f64x8::splat(0.0018891666666666666) * t555;
            let t557 = t56 * t213;
            let t558 = f64x8::splat(1.0) / t261;
            let t559 = t558 * t352;
            let t560 = t557 * t559;
            let t561 = t11 * t560;
            let t563 = t556 + f64x8::splat(0.0018891666666666666) * t561;
            let t564 = t203 * t563;
            let t565 = t564 * t184;
            let t567 = f64x8::splat(2.0) / f64x8::splat(15.0) * t565 * t221;
            let t568 = t514 * t220;
            let t570 = f64x8::splat(4.0) / f64x8::splat(45.0) * t211 * t568;
            let t571 = t210 * t518;
            let t573 = f64x8::splat(1.0) / t212 / t204;
            let t574 = t22 * t573;
            let t575 = t219 * t558;
            let t576 = t575 * t352;
            let t577 = t574 * t576;
            let t579 = f64x8::splat(4.0) / f64x8::splat(45.0) * t571 * t577;
            let t580 = t218 * t218;
            let t581 = f64x8::splat(1.0) / t580;
            let t582 = t213 * t581;
            let t583 = f64x8::splat(0.035991666666666665) * t555;
            let t587 = f64x8::splat(0.006666666666666667) * t190 * t533 * t212;
            let t588 = f64x8::splat(1.0) / t205;
            let t589 = t191 * t588;
            let t590 = t589 * t559;
            let t593 = -t583 - f64x8::splat(0.035991666666666665) * t561 - t587 - f64x8::splat(0.006666666666666667) * t25 * t590;
            let t594 = t582 * t593;
            let t595 = t186 * t594;
            let t597 = f64x8::splat(2.0) / f64x8::splat(15.0) * t211 * t595;
            let t598 = t465 * t225;
            let t602 = t147 * t1 * t3;
            let t603 = t225 * t229;
            let t604 = t10 * t603;
            let t606 = f64x8::splat(0.10821041362364843) * t602 * t604;
            let t609 = f64x8::splat(0.4125) * t325 - t331 / f64x8::splat(6.0);
            let t610 = t609 * f64x8::splat(M_PI);
            let t611 = t610 * t7;
            let t613 = f64x8::splat(4.0) / f64x8::splat(3.0) * t226 * t611;
            let t615 = f64x8::splat(4.0) / f64x8::splat(3.0) * t226 * t230;
            let t616 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + f64x8::splat(4.0) / f64x8::splat(3.0) * t598 * t231 + t606 + t613 + t615;
            let t624 = f64x8::splat(0.0837628205355044) * t458 * t242;
            let t628 = t483 * t3;
            let t629 = t156 * t161;
            let t632 = t478 / f64x8::splat(2.0) + f64x8::splat(0.03135) * t628 * t629;
            let t634 = f64x8::splat(0.0837628205355044) * t148 * t632;
            let t635 = t5 * t299;
            let t638 = f64x8::splat(0.019897291109174608) * t168 * t635 * t270;
            let t639 = t465 * t247;
            let t640 = t639 * t251;
            let t643 = t147 * t19;
            let t644 = f64x8::splat(1.0) / t3;
            let t645 = t643 * t644;
            let t646 = t247 * t251;
            let t647 = t24 * t646;
            let t649 = f64x8::splat(0.06077777777777778) * t645 * t647;
            let t652 = f64x8::splat(0.03233333333333333) * t325 - f64x8::splat(0.11266666666666666) * t331;
            let t653 = t248 * t652;
            let t655 = t653 * t256 / f64x8::splat(3.0);
            let t656 = t254 * t10;
            let t658 = f64x8::splat(2.0) / f64x8::splat(9.0) * t252 * t656;
            let t659 = t92 * t43;
            let t661 = t93 * t50;
            let t665 = (f64x8::splat(4.0) / f64x8::splat(3.0) * t659 * t348 + f64x8::splat(4.0) / f64x8::splat(3.0) * t661 * t352) * t108;
            let t668 = t266 * t9;
            let t670 = f64x8::splat(2.0) / f64x8::splat(45.0) * t265 * t668;
            let t671 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + t640 * t256 / f64x8::splat(3.0) + t649 + t655 + t658 - t665 * t267 / f64x8::splat(15.0) - t670;
            let t677 = f64x8::splat(0.5694518669548363) * t153 * t474 * t274;
            let t678 = t316 + t337 + t341 - t346 + t359 + t392 + t394 - t399 - t404;
            let t684 = t168 * t635;
            let t686 = f64x8::splat(0.019957056683757683) * t684 * t286;
            let t688 = t465 * t159 * t285;
            let t692 = t147 * t477 * t285;
            let t694 = f64x8::splat(0.01197423401025461) * t281 * t692;
            let t695 = t462 * t147;
            let t698 = f64x8::splat(0.0002905674151788692) * t695 * t159 * t285;
            let t699 = t299 * t147;
            let t702 = f64x8::splat(0.053059442957798957) * t169 * t699 * t242;
            let t703 = t171 * t465;
            let t709 = f64x8::splat(0.031835665774679375) * t169 * t289 * t632;
            let t711 = f64x8::splat(0.10665013548435875) * t462 * t274;
            let t717 = f64x8::splat(1.0) / t22 / t461;
            let t721 = f64x8::splat(0.054045904796391424) * t169 * t717 * t274 * t301;
            let t726 = f64x8::splat(3.0) * t405 * t143 * t411 + t453 * t456 + (t460 - f64x8::splat(0.031505407223141116) * t466 * t164 - t481 - t487 + t491 - f64x8::splat(0.005388405304614574) * t169 * t171 * t616 * t163) * t125 + (t624 - f64x8::splat(0.0837628205355044) * t466 * t242 - t634 + t638 - f64x8::splat(0.011938374665504766) * t168 * t245 * t671 - t677 + f64x8::splat(0.42708890021612717) * t153 * t156 * t678) * t279 + t686 - f64x8::splat(0.01197423401025461) * t281 * t688 - t694 - t698 + (t702 - f64x8::splat(0.031835665774679375) * t169 * t703 * t242 - t709 - t711 + f64x8::splat(0.05332506774217938) * t145 * t678) * t296 - t721 + f64x8::splat(0.020267214298646783) * t169 * t299 * t678 * t301;
            let t727 = t726 * t312;
            let t729 = f64x8::splat(1.0) / t311 / t309;
            let t731 = t305 * t729 * t19;
            let t732 = t20 * t5;
            let t734 = t732 * t171 * t116;
            let t735 = t731 * t734;
            let t736 = f64x8::splat(0.41076328840066667) * t735;
            let t737 = t316 + t337 + t341 - t346 + t359 + t392 + t394 - t399 - t404 - t727 - t736;
            let tvrho0 = t7 * t737 - t313 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t739 = -t41 - t347;
            let t742 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t739));
            let t743 = -t739;
            let t746 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t743));
            let t748 = (t742 + t746) * t59;
            let t749 = t748 * t87;
            let t750 = t40 * t749;
            let t751 = t748 * t85;
            let t752 = f64x8::splat(0.019751789702565206) * t751;
            let t753 = t406 * t739;
            let t754 = t408 * t743;
            let t756 = t753 / f64x8::splat(3.0) + t754 / f64x8::splat(3.0);
            let t757 = t143 * t756;
            let t760 = t120 * t756;
            let t762 = f64x8::splat(2.923025) * t102 * t760;
            let t763 = t128 * t756;
            let t764 = t10 * t763;
            let t767 = t436 * t756;
            let t770 = -t418 - t762 - t425 - t426 * t764 / f64x8::splat(2.0) - t434 - f64x8::splat(1.46904) * t127 * t767;
            let t774 = -t418 - t762 - t447 - f64x8::splat(1.724255) * t133 * t764;
            let t776 = t770 * t138 - t444 * t774;
            let t777 = t101 * t776;
            let t780 = f64x8::splat(2.0) * t347 + f64x8::splat(2.0) * t463;
            let t781 = t41 * t780;
            let t784 = t739 / f64x8::splat(2.0);
            let t785 = t198 * t784;
            let t786 = t186 * t785;
            let t788 = f64x8::splat(4.0) / f64x8::splat(15.0) * t493 * t786;
            let t789 = t504 * t739;
            let t790 = t503 * t789;
            let t791 = t11 * t790;
            let t793 = t502 + f64x8::splat(0.0018891666666666666) * t791;
            let t794 = t173 * t793;
            let t795 = t794 * t184;
            let t797 = f64x8::splat(2.0) / f64x8::splat(15.0) * t795 * t199;
            let t798 = t523 * t739;
            let t799 = t522 * t798;
            let t801 = f64x8::splat(4.0) / f64x8::splat(45.0) * t519 * t799;
            let t803 = t538 * t789;
            let t806 = -t531 - f64x8::splat(0.035991666666666665) * t791 - t536 - f64x8::splat(0.006666666666666667) * t25 * t803;
            let t807 = t530 * t806;
            let t808 = t186 * t807;
            let t810 = f64x8::splat(2.0) / f64x8::splat(15.0) * t185 * t808;
            let t811 = -t784;
            let t812 = t220 * t811;
            let t813 = t186 * t812;
            let t815 = f64x8::splat(4.0) / f64x8::splat(15.0) * t548 * t813;
            let t816 = t558 * t743;
            let t817 = t557 * t816;
            let t818 = t11 * t817;
            let t820 = t556 + f64x8::splat(0.0018891666666666666) * t818;
            let t821 = t203 * t820;
            let t822 = t821 * t184;
            let t824 = f64x8::splat(2.0) / f64x8::splat(15.0) * t822 * t221;
            let t825 = t575 * t743;
            let t826 = t574 * t825;
            let t828 = f64x8::splat(4.0) / f64x8::splat(45.0) * t571 * t826;
            let t830 = t589 * t816;
            let t833 = -t583 - f64x8::splat(0.035991666666666665) * t818 - t587 - f64x8::splat(0.006666666666666667) * t25 * t830;
            let t834 = t582 * t833;
            let t835 = t186 * t834;
            let t837 = f64x8::splat(2.0) / f64x8::splat(15.0) * t211 * t835;
            let t838 = t780 * t225;
            let t841 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + f64x8::splat(4.0) / f64x8::splat(3.0) * t838 * t231 + t606 + t613 + t615;
            let t850 = t780 * t247;
            let t851 = t850 * t251;
            let t858 = (f64x8::splat(4.0) / f64x8::splat(3.0) * t659 * t739 + f64x8::splat(4.0) / f64x8::splat(3.0) * t661 * t743) * t108;
            let t861 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + t851 * t256 / f64x8::splat(3.0) + t649 + t655 + t658 - t858 * t267 / f64x8::splat(15.0) - t670;
            let t865 = t316 + t337 - t341 - t346 + t750 + t392 + t752 - t399 - t404;
            let t872 = t780 * t159 * t285;
            let t875 = t171 * t780;
            let t887 = f64x8::splat(3.0) * t405 * t757 + t777 * t456 + (t460 - f64x8::splat(0.031505407223141116) * t781 * t164 - t481 - t487 + t491 - f64x8::splat(0.005388405304614574) * t169 * t171 * t841 * t163) * t125 + (t624 - f64x8::splat(0.0837628205355044) * t781 * t242 - t634 + t638 - f64x8::splat(0.011938374665504766) * t168 * t245 * t861 - t677 + f64x8::splat(0.42708890021612717) * t153 * t156 * t865) * t279 + t686 - f64x8::splat(0.01197423401025461) * t281 * t872 - t694 - t698 + (t702 - f64x8::splat(0.031835665774679375) * t169 * t875 * t242 - t709 - t711 + f64x8::splat(0.05332506774217938) * t145 * t865) * t296 - t721 + f64x8::splat(0.020267214298646783) * t169 * t299 * t865 * t301;
            let t888 = t887 * t312;
            let t889 = t316 + t337 - t341 - t346 + t750 + t392 + t752 - t399 - t404 - t888 - t736;
            let tvrho1 = t7 * t889 - t313 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
