//! GGA_C_ZPBEINT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zpbeint.c`
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
pub fn gga_c_zpbeint_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
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
            let t11 = t4 * t6 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t5 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
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
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.0310907) * t62 * t71 + t33 - f64x8::splat(0.0197516734986138) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.0197516734986138) * t60 * t85;
            let t92 = t45 * t45;
            let t93 = t47 * t47;
            let t94 = ((t44).select(t92, t93));
            let t95 = t52 * t52;
            let t96 = ((t51).select(t92, t95));
            let t98 = t94 / f64x8::splat(2.0) + t96 / f64x8::splat(2.0);
            let t100 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t101 = ((t100).sqrt());
            let t102 = t101 * t100;
            let t103 = param_alpha * t102;
            let t104 = t98 * t98;
            let t105 = t104 * t98;
            let t106 = f64x8::splat(1.0) / t105;
            let t109 = f64x8::splat(1.0) / t14 / t11;
            let t113 = (simd::pow(t98, t103 * t39 * t106 * t109 / f64x8::splat(16.0)));
            let t114 = (simd::ln(f64x8::splat(2.0)));
            let t115 = f64x8::splat(1.0) - t114;
            let t116 = t113 * t115;
            let t117 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t118 * t105;
            let t121 = f64x8::splat(1.0) / t8 / t37;
            let t122 = t100 * t121;
            let t124 = f64x8::splat(1.0) / t104;
            let t126 = f64x8::splat(1.0) / t3;
            let t127 = t126 * t5;
            let t128 = t124 * t19 * t127;
            let t131 = f64x8::splat(1.0) / t115;
            let t132 = param_beta * t131;
            let t134 = (-t33 + t89 + t91) * t131;
            let t135 = t117 * t106;
            let t137 = (simd::exp(-t134 * t135));
            let t138 = t137 - f64x8::splat(1.0);
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t117 * t139;
            let t141 = t100 * t100;
            let t143 = t132 * t140 * t141;
            let t145 = f64x8::splat(1.0) / t22 / t38;
            let t146 = t56 * t56;
            let t147 = t145 * t146;
            let t148 = t104 * t104;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t147 * t149;
            let t151 = f64x8::splat(1.0) / t20;
            let t152 = t1 * t151;
            let t153 = t152 * t6;
            let t154 = t150 * t153;
            let t157 = t122 * t56 * t128 / f64x8::splat(96.0) + t143 * t154 / f64x8::splat(3072.0);
            let t158 = param_beta * t157;
            let t159 = t131 * t117;
            let t162 = t132 * t140 * t157 + f64x8::splat(1.0);
            let t163 = f64x8::splat(1.0) / t162;
            let t164 = t159 * t163;
            let t166 = t158 * t164 + f64x8::splat(1.0);
            let t167 = (simd::ln(t166));
            let t168 = t119 * t167;
            let t169 = t116 * t168;
            let tzk0 = -t33 + t89 + t91 + t169;
            acc_zk = tzk0;
            let t171 = f64x8::splat(1.0) / t8 / t7;
            let t172 = t6 * t171;
            let t174 = t4 * t172 * t31;
            let t175 = f64x8::splat(0.0011073470983333333) * t174;
            let t176 = t27 * t27;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t13 * t177;
            let t180 = f64x8::splat(1.0) / t14 * t1;
            let t181 = t3 * t6;
            let t182 = t181 * t171;
            let t183 = t180 * t182;
            let t185 = t4 * t172;
            let t187 = ((t11).sqrt());
            let t188 = t187 * t1;
            let t189 = t188 * t182;
            let t193 = t5 / t22 / t7;
            let t194 = t21 * t193;
            let t196 = -f64x8::splat(0.632975) * t183 - f64x8::splat(0.29896666666666666) * t185 - f64x8::splat(0.1023875) * t189 - f64x8::splat(0.08215666666666667) * t194;
            let t197 = f64x8::splat(1.0) / t30;
            let t198 = t196 * t197;
            let t199 = t178 * t198;
            let t200 = f64x8::splat(1.0) * t199;
            let t201 = t35 * t34;
            let t202 = t201 * t39;
            let t203 = t202 * t88;
            let t204 = f64x8::splat(4.0) * t203;
            let t205 = t38 * t7;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t36 * t206;
            let t208 = t207 * t88;
            let t209 = f64x8::splat(4.0) * t208;
            let t210 = f64x8::splat(1.0) / t37;
            let t211 = t34 * t210;
            let t212 = t41 - t211;
            let t215 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t212));
            let t216 = -t212;
            let t219 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t216));
            let t221 = (t215 + t219) * t59;
            let t222 = t221 * t87;
            let t223 = t40 * t222;
            let t227 = t67 * t67;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t62 * t228;
            let t234 = -f64x8::splat(1.176575) * t183 - f64x8::splat(0.516475) * t185 - f64x8::splat(0.2103875) * t189 - f64x8::splat(0.104195) * t194;
            let t235 = f64x8::splat(1.0) / t70;
            let t236 = t234 * t235;
            let t242 = t80 * t80;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t75 * t243;
            let t249 = -f64x8::splat(0.8630833333333333) * t183 - f64x8::splat(0.301925) * t185 - f64x8::splat(0.05501625) * t189 - f64x8::splat(0.082785) * t194;
            let t250 = f64x8::splat(1.0) / t83;
            let t251 = t249 * t250;
            let t254 = f64x8::splat(0.0005323764196666666) * t4 * t172 * t71 + f64x8::splat(1.0) * t229 * t236 - t175 - t200 + f64x8::splat(0.00018311447306006544) * t4 * t172 * t84 + f64x8::splat(0.5848223622634646) * t244 * t251;
            let t255 = t60 * t254;
            let t256 = t40 * t255;
            let t257 = t221 * t85;
            let t258 = f64x8::splat(0.0197516734986138) * t257;
            let t259 = t60 * t1;
            let t261 = t181 * t171 * t84;
            let t262 = t259 * t261;
            let t263 = f64x8::splat(0.00018311447306006544) * t262;
            let t264 = t60 * t75;
            let t266 = t243 * t249 * t250;
            let t267 = t264 * t266;
            let t268 = f64x8::splat(0.5848223622634646) * t267;
            let t272 = t103 * t206 * t106 * t109 / f64x8::splat(4.0);
            let t273 = t103 * t39;
            let t274 = t149 * t109;
            let t275 = f64x8::splat(1.0) / t47;
            let t278 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t275 * t212));
            let t279 = f64x8::splat(1.0) / t52;
            let t282 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t279 * t216));
            let t284 = t278 / f64x8::splat(2.0) + t282 / f64x8::splat(2.0);
            let t285 = t274 * t284;
            let t286 = t273 * t285;
            let t289 = f64x8::splat(1.0) / t8 / t205;
            let t294 = f64x8::splat(1.0) / t14 / t25 / f64x8::splat(4.0);
            let t296 = t294 * t1 * t181;
            let t298 = t103 * t289 * t106 * t296 / f64x8::splat(32.0);
            let t299 = -t272 - f64x8::splat(3.0) / f64x8::splat(16.0) * t286 + t298;
            let t300 = (simd::ln(t98));
            let t303 = t299 * t300 + t286 / f64x8::splat(16.0);
            let t304 = t113 * t303;
            let t305 = t304 * t115;
            let t306 = t305 * t168;
            let t307 = t116 * t118;
            let t308 = t104 * t167;
            let t310 = t307 * t308 * t284;
            let t311 = f64x8::splat(3.0) * t310;
            let t312 = t37 * t7;
            let t314 = f64x8::splat(1.0) / t8 / t312;
            let t315 = t100 * t314;
            let t318 = f64x8::splat(7.0) / f64x8::splat(288.0) * t315 * t56 * t128;
            let t319 = t56 * t106;
            let t320 = t122 * t319;
            let t321 = t19 * t126;
            let t322 = t5 * t284;
            let t323 = t321 * t322;
            let t326 = t132 * t117;
            let t327 = t138 * t138;
            let t328 = f64x8::splat(1.0) / t327;
            let t329 = t328 * t141;
            let t331 = t326 * t329 * t145;
            let t332 = t146 * t149;
            let t333 = t332 * t1;
            let t334 = t151 * t6;
            let t336 = (t175 + t200 + t204 - t209 + t223 + t256 + t258 - t263 - t268) * t131;
            let t338 = t117 * t149;
            let t339 = t338 * t284;
            let t342 = f64x8::splat(3.0) * t134 * t339 - t135 * t336;
            let t343 = t342 * t137;
            let t344 = t334 * t343;
            let t345 = t333 * t344;
            let t349 = f64x8::splat(1.0) / t22 / t205;
            let t350 = t349 * t146;
            let t351 = t350 * t149;
            let t352 = t351 * t153;
            let t354 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t143 * t352;
            let t355 = t139 * t141;
            let t357 = t326 * t355 * t145;
            let t359 = f64x8::splat(1.0) / t148 / t98;
            let t360 = t146 * t359;
            let t361 = t360 * t1;
            let t363 = t361 * t334 * t284;
            let t366 = -t318 - t320 * t323 / f64x8::splat(48.0) - t331 * t345 / f64x8::splat(3072.0) - t354 - t357 * t363 / f64x8::splat(768.0);
            let t367 = param_beta * t366;
            let t369 = t158 * t131;
            let t370 = t162 * t162;
            let t371 = f64x8::splat(1.0) / t370;
            let t372 = t117 * t371;
            let t373 = t328 * t157;
            let t378 = t132 * t140 * t366 - t326 * t343 * t373;
            let t379 = t372 * t378;
            let t381 = t164 * t367 - t369 * t379;
            let t382 = t105 * t381;
            let t383 = f64x8::splat(1.0) / t166;
            let t385 = t307 * t382 * t383;
            let t386 = t175 + t200 + t204 - t209 + t223 + t256 + t258 - t263 - t268 + t306 + t311 + t385;
            let tvrho0 = t386 * t7 + t169 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t388 = -t41 - t211;
            let t391 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t388));
            let t392 = -t388;
            let t395 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t392));
            let t397 = (t391 + t395) * t59;
            let t398 = t397 * t87;
            let t399 = t40 * t398;
            let t400 = t397 * t85;
            let t401 = f64x8::splat(0.0197516734986138) * t400;
            let t404 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t275 * t388));
            let t407 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t279 * t392));
            let t409 = t404 / f64x8::splat(2.0) + t407 / f64x8::splat(2.0);
            let t410 = t274 * t409;
            let t411 = t273 * t410;
            let t413 = -t272 - f64x8::splat(3.0) / f64x8::splat(16.0) * t411 + t298;
            let t416 = t413 * t300 + t411 / f64x8::splat(16.0);
            let t417 = t113 * t416;
            let t418 = t417 * t115;
            let t419 = t418 * t168;
            let t421 = t307 * t308 * t409;
            let t422 = f64x8::splat(3.0) * t421;
            let t423 = t5 * t409;
            let t424 = t321 * t423;
            let t428 = (t175 + t200 - t204 - t209 + t399 + t256 + t401 - t263 - t268) * t131;
            let t430 = t338 * t409;
            let t433 = f64x8::splat(3.0) * t134 * t430 - t135 * t428;
            let t434 = t433 * t137;
            let t435 = t334 * t434;
            let t436 = t333 * t435;
            let t440 = t361 * t334 * t409;
            let t443 = -t318 - t320 * t424 / f64x8::splat(48.0) - t331 * t436 / f64x8::splat(3072.0) - t354 - t357 * t440 / f64x8::splat(768.0);
            let t444 = param_beta * t443;
            let t450 = t132 * t140 * t443 - t326 * t373 * t434;
            let t451 = t372 * t450;
            let t453 = t164 * t444 - t369 * t451;
            let t454 = t105 * t453;
            let t456 = t307 * t454 * t383;
            let t457 = t175 + t200 - t204 - t209 + t399 + t256 + t401 - t263 - t268 + t419 + t422 + t456;
            let tvrho1 = t457 * t7 + t169 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t459 = t113 * param_alpha;
            let t460 = t101 * t39;
            let t462 = t109 * t300;
            let t463 = t115 * t118;
            let t464 = t463 * t167;
            let t465 = t462 * t464;
            let t466 = t459 * t460 * t465;
            let t467 = f64x8::splat(3.0) / f64x8::splat(32.0) * t466;
            let t468 = t121 * t56;
            let t470 = t321 * t5;
            let t471 = t468 * t124 * t470;
            let t474 = t132 * t140 * t100;
            let t475 = t474 * t154;
            let t477 = t471 / f64x8::splat(96.0) + t475 / f64x8::splat(1536.0);
            let t478 = param_beta * t477;
            let t480 = param_beta * param_beta;
            let t481 = t480 * t157;
            let t482 = t115 * t115;
            let t483 = f64x8::splat(1.0) / t482;
            let t484 = t481 * t483;
            let t485 = t117 * t117;
            let t486 = t485 * t371;
            let t488 = t486 * t139 * t477;
            let t490 = t164 * t478 - t484 * t488;
            let t491 = t105 * t490;
            let t493 = t307 * t491 * t383;
            let tvsigma0 = t7 * (t467 + t493);
            acc_vsigma_0 = tvsigma0;
            let t495 = f64x8::splat(3.0) / f64x8::splat(16.0) * t466;
            let t498 = t471 / f64x8::splat(48.0) + t475 / f64x8::splat(768.0);
            let t499 = param_beta * t498;
            let t502 = t486 * t139 * t498;
            let t504 = t164 * t499 - t484 * t502;
            let t505 = t105 * t504;
            let t507 = t307 * t505 * t383;
            let tvsigma1 = t7 * (t495 + t507);
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
