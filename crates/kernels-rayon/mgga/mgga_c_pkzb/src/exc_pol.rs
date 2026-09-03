//! MGGA_C_PKZB exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_pkzb.c`
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
pub fn mgga_c_pkzb_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = v_rho0 * v_rho0;
            let t3 = (simd::cbrt(v_rho0));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / t2;
            let t7 = v_sigma0 * t6;
            let t8 = v_rho0 - v_rho1;
            let t9 = v_rho0 + v_rho1;
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t8 * t10;
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = t12 / f64x8::splat(2.0);
            let t14 = (simd::cbrt(t13));
            let t15 = t14 * t14;
            let t16 = t15 * t13;
            let t18 = v_rho1 * v_rho1;
            let t19 = (simd::cbrt(v_rho1));
            let t20 = t19 * t19;
            let t22 = f64x8::splat(1.0) / t20 / t18;
            let t23 = v_sigma2 * t22;
            let t24 = f64x8::splat(1.0) - t11;
            let t25 = t24 / f64x8::splat(2.0);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t26;
            let t28 = t27 * t25;
            let t30 = t16 * t7 + t23 * t28;
            let t31 = t30 * t30;
            let t33 = f64x8::splat(1.0) / t4 / v_rho0;
            let t34 = v_tau0 * t33;
            let t37 = f64x8::splat(1.0) / t20 / v_rho1;
            let t38 = v_tau1 * t37;
            let t40 = t16 * t34 + t28 * t38;
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.00828125) * t31 * t42;
            let t46 = f64x8::splat(M_CBRT3);
            let t47 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t48 = (simd::cbrt(t47));
            let t49 = t46 * t48;
            let t50 = f64x8::splat(M_CBRT4);
            let t51 = t50 * t50;
            let t52 = (simd::cbrt(t9));
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t51 * t53;
            let t55 = t49 * t54;
            let t57 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t55;
            let t58 = ((t55).sqrt());
            let t61 = ((t55) * (t55).sqrt());
            let t63 = t46 * t46;
            let t64 = t48 * t48;
            let t65 = t63 * t64;
            let t66 = t52 * t52;
            let t67 = f64x8::splat(1.0) / t66;
            let t68 = t50 * t67;
            let t69 = t65 * t68;
            let t71 = f64x8::splat(3.79785) * t58 + f64x8::splat(0.8969) * t55 + f64x8::splat(0.204775) * t61 + f64x8::splat(0.123235) * t69;
            let t74 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t71;
            let t75 = (simd::ln(t74));
            let t77 = f64x8::splat(0.0621814) * t57 * t75;
            let t78 = t8 * t8;
            let t79 = t78 * t78;
            let t80 = t9 * t9;
            let t81 = t80 * t80;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t79 * t82;
            let t84 = (t12).simd_le(zeta_threshold);
            let t85 = (simd::cbrt(zeta_threshold));
            let t86 = t85 * zeta_threshold;
            let t87 = (simd::cbrt(t12));
            let t88 = t87 * t12;
            let t89 = ((t84).select(t86, t88));
            let t90 = (t24).simd_le(zeta_threshold);
            let t91 = (simd::cbrt(t24));
            let t92 = t91 * t24;
            let t93 = ((t90).select(t86, t92));
            let t94 = t89 + t93 - f64x8::splat(2.0);
            let t95 = f64x8::splat(M_CBRT2);
            let t98 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t95 - f64x8::splat(2.0));
            let t99 = t94 * t98;
            let t101 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t55;
            let t106 = f64x8::splat(7.05945) * t58 + f64x8::splat(1.549425) * t55 + f64x8::splat(0.420775) * t61 + f64x8::splat(0.1562925) * t69;
            let t109 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t106;
            let t110 = (simd::ln(t109));
            let t114 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t55;
            let t119 = f64x8::splat(5.1785) * t58 + f64x8::splat(0.905775) * t55 + f64x8::splat(0.1100325) * t61 + f64x8::splat(0.1241775) * t69;
            let t122 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t119;
            let t123 = (simd::ln(t122));
            let t124 = t114 * t123;
            let t126 = -f64x8::splat(0.0310907) * t101 * t110 + t77 - f64x8::splat(0.0197516734986138) * t124;
            let t127 = t99 * t126;
            let t128 = t83 * t127;
            let t130 = f64x8::splat(0.0197516734986138) * t99 * t124;
            let t131 = (simd::ln(f64x8::splat(2.0)));
            let t132 = f64x8::splat(1.0) - t131;
            let t133 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t132 * t134;
            let t136 = t85 * t85;
            let t137 = t87 * t87;
            let t138 = ((t84).select(t136, t137));
            let t139 = t91 * t91;
            let t140 = ((t90).select(t136, t139));
            let t142 = t138 / f64x8::splat(2.0) + t140 / f64x8::splat(2.0);
            let t143 = t142 * t142;
            let t144 = t143 * t142;
            let t146 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t148 = f64x8::splat(1.0) / t52 / t80;
            let t149 = t146 * t148;
            let t151 = f64x8::splat(1.0) / t143;
            let t153 = f64x8::splat(1.0) / t48;
            let t154 = t153 * t50;
            let t155 = t151 * t63 * t154;
            let t158 = f64x8::splat(1.0) / t132;
            let t160 = (-t77 + t128 + t130) * t158;
            let t161 = f64x8::splat(1.0) / t144;
            let t162 = t133 * t161;
            let t164 = (simd::exp(-t160 * t162));
            let t165 = t164 - f64x8::splat(1.0);
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t158 * t166;
            let t168 = t146 * t146;
            let t170 = f64x8::splat(1.0) / t66 / t81;
            let t171 = t168 * t170;
            let t173 = t95 * t95;
            let t174 = t143 * t143;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t173 * t175;
            let t177 = f64x8::splat(1.0) / t64;
            let t178 = t46 * t177;
            let t179 = t178 * t51;
            let t180 = t176 * t179;
            let t183 = t149 * t95 * t155 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t167 * t171 * t180;
            let t184 = t183 * t158;
            let t187 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t167 * t183;
            let t188 = f64x8::splat(1.0) / t187;
            let t191 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t184 * t188;
            let t192 = (simd::ln(t191));
            let t195 = t135 * t144 * t192 + t128 + t130 - t77;
            let t196 = t45 * t195;
            let t197 = v_sigma0 * v_sigma0;
            let t198 = f64x8::splat(1.0) / t2;
            let t199 = t197 * t198;
            let t200 = v_tau0 * v_tau0;
            let t201 = f64x8::splat(1.0) / t200;
            let t203 = ((v_rho0).simd_le(dens_threshold)) | (t84);
            let t204 = t49 * t51;
            let t205 = t53 * t95;
            let t206 = f64x8::splat(1.0) / t85;
            let t207 = f64x8::splat(1.0) / t87;
            let t208 = ((t84).select(t206, t207));
            let t210 = t204 * t205 * t208;
            let t212 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t210;
            let t213 = ((t210).sqrt());
            let t216 = ((t210) * (t210).sqrt());
            let t218 = t65 * t50;
            let t219 = t67 * t173;
            let t220 = t208 * t208;
            let t222 = t218 * t219 * t220;
            let t224 = f64x8::splat(3.79785) * t213 + f64x8::splat(0.8969) * t210 + f64x8::splat(0.204775) * t216 + f64x8::splat(0.123235) * t222;
            let t227 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t224;
            let t228 = (simd::ln(t227));
            let t230 = f64x8::splat(0.0621814) * t212 * t228;
            let t231 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t233 = ((t231).select(t86, f64x8::splat(2.0) * t95));
            let t234 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t235 = ((t234).select(t86, f64x8::splat(0.0)));
            let t237 = (t233 + t235 - f64x8::splat(2.0)) * t98;
            let t239 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t210;
            let t244 = f64x8::splat(7.05945) * t213 + f64x8::splat(1.549425) * t210 + f64x8::splat(0.420775) * t216 + f64x8::splat(0.1562925) * t222;
            let t247 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t244;
            let t248 = (simd::ln(t247));
            let t252 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t210;
            let t257 = f64x8::splat(5.1785) * t213 + f64x8::splat(0.905775) * t210 + f64x8::splat(0.1100325) * t216 + f64x8::splat(0.1241775) * t222;
            let t260 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t257;
            let t261 = (simd::ln(t260));
            let t262 = t252 * t261;
            let t265 = t237 * (-f64x8::splat(0.0310907) * t239 * t248 + t230 - f64x8::splat(0.0197516734986138) * t262);
            let t267 = f64x8::splat(0.0197516734986138) * t237 * t262;
            let t268 = ((t231).select(t136, t173));
            let t269 = ((t234).select(t136, f64x8::splat(0.0)));
            let t271 = t268 / f64x8::splat(2.0) + t269 / f64x8::splat(2.0);
            let t272 = t271 * t271;
            let t273 = t272 * t271;
            let t274 = f64x8::splat(1.0) / t272;
            let t275 = t274 * t63;
            let t276 = t7 * t275;
            let t277 = f64x8::splat(1.0) / t208;
            let t279 = t154 * t52 * t277;
            let t284 = f64x8::splat(1.0) / t273;
            let t285 = t133 * t284;
            let t287 = (simd::exp(-(-t230 + t265 + t267) * t158 * t285));
            let t288 = t287 - f64x8::splat(1.0);
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t158 * t289;
            let t291 = t2 * t2;
            let t292 = t291 * v_rho0;
            let t294 = f64x8::splat(1.0) / t3 / t292;
            let t295 = t197 * t294;
            let t296 = t272 * t272;
            let t297 = f64x8::splat(1.0) / t296;
            let t299 = t290 * t295 * t297;
            let t300 = t51 * t66;
            let t301 = f64x8::splat(1.0) / t220;
            let t302 = t300 * t301;
            let t303 = t178 * t302;
            let t306 = t276 * t279 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t299 * t303;
            let t307 = t306 * t158;
            let t310 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t290 * t306;
            let t311 = f64x8::splat(1.0) / t310;
            let t314 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t307 * t311;
            let t315 = (simd::ln(t314));
            let t318 = t135 * t273 * t315 - t230 + t265 + t267;
            let t319 = ((t84).select(zeta_threshold, t12));
            let t322 = ((t203).select(f64x8::splat(0.0), t318 * t319 / f64x8::splat(2.0)));
            let t323 = t201 * t322;
            let t325 = f64x8::splat(0.02390625) * t199 * t323;
            let t326 = v_sigma2 * v_sigma2;
            let t327 = f64x8::splat(1.0) / t18;
            let t328 = t326 * t327;
            let t329 = v_tau1 * v_tau1;
            let t330 = f64x8::splat(1.0) / t329;
            let t332 = ((v_rho1).simd_le(dens_threshold)) | (t90);
            let t333 = f64x8::splat(1.0) / t91;
            let t334 = ((t90).select(t206, t333));
            let t336 = t204 * t205 * t334;
            let t338 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t336;
            let t339 = ((t336).sqrt());
            let t342 = ((t336) * (t336).sqrt());
            let t344 = t334 * t334;
            let t346 = t218 * t219 * t344;
            let t348 = f64x8::splat(3.79785) * t339 + f64x8::splat(0.8969) * t336 + f64x8::splat(0.204775) * t342 + f64x8::splat(0.123235) * t346;
            let t351 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t348;
            let t352 = (simd::ln(t351));
            let t354 = f64x8::splat(0.0621814) * t338 * t352;
            let t356 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t336;
            let t361 = f64x8::splat(7.05945) * t339 + f64x8::splat(1.549425) * t336 + f64x8::splat(0.420775) * t342 + f64x8::splat(0.1562925) * t346;
            let t364 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t361;
            let t365 = (simd::ln(t364));
            let t369 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t336;
            let t374 = f64x8::splat(5.1785) * t339 + f64x8::splat(0.905775) * t336 + f64x8::splat(0.1100325) * t342 + f64x8::splat(0.1241775) * t346;
            let t377 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t374;
            let t378 = (simd::ln(t377));
            let t379 = t369 * t378;
            let t382 = t237 * (-f64x8::splat(0.0310907) * t356 * t365 + t354 - f64x8::splat(0.0197516734986138) * t379);
            let t384 = f64x8::splat(0.0197516734986138) * t237 * t379;
            let t385 = t23 * t275;
            let t386 = f64x8::splat(1.0) / t334;
            let t388 = t154 * t52 * t386;
            let t394 = (simd::exp(-(-t354 + t382 + t384) * t158 * t285));
            let t395 = t394 - f64x8::splat(1.0);
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t158 * t396;
            let t398 = t18 * t18;
            let t399 = t398 * v_rho1;
            let t401 = f64x8::splat(1.0) / t19 / t399;
            let t402 = t326 * t401;
            let t404 = t397 * t402 * t297;
            let t405 = f64x8::splat(1.0) / t344;
            let t406 = t300 * t405;
            let t407 = t178 * t406;
            let t410 = t385 * t388 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t404 * t407;
            let t411 = t410 * t158;
            let t414 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t397 * t410;
            let t415 = f64x8::splat(1.0) / t414;
            let t418 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t411 * t415;
            let t419 = (simd::ln(t418));
            let t422 = t135 * t273 * t419 - t354 + t382 + t384;
            let t423 = ((t90).select(zeta_threshold, t24));
            let t426 = ((t332).select(f64x8::splat(0.0), t422 * t423 / f64x8::splat(2.0)));
            let t427 = t330 * t426;
            let t429 = f64x8::splat(0.02390625) * t328 * t427;
            let tzk0 = t196 - t325 - t429;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
