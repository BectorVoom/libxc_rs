//! MGGA_C_KCIS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_kcis.c`
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
pub fn mgga_c_kcis_exc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t7 * t10;
            let t12 = t5 * t11;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t6 * t24;
            let t26 = t22 * t25;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.062182) * t14 * t32;
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * zeta_threshold;
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(1.0)));
            let t41 = f64x8::splat(M_CBRT2);
            let t44 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t41 - f64x8::splat(2.0));
            let t45 = (f64x8::splat(2.0) * t38 - f64x8::splat(2.0)) * t44;
            let t47 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t52 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t55 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t52;
            let t56 = (simd::ln(t55));
            let t57 = t47 * t56;
            let t60 = -t34 + f64x8::splat(0.019751789702565206) * t45 * t57;
            let t62 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t63 = t41 * t62;
            let t64 = t8 * t8;
            let t66 = f64x8::splat(1.0) / t9 / t64;
            let t68 = f64x8::splat(1.0) / t4;
            let t69 = t20 * t68;
            let t70 = (f64x8::splat(0.0)).simd_lt(t60);
            let t72 = ((t70).select(t60, -t60));
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = t6 * t73;
            let t75 = t69 * t74;
            let t78 = f64x8::splat(1.0) + t63 * t66 * t75 / f64x8::splat(96.0);
            let t79 = (simd::ln(t78));
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t79;
            let t82 = f64x8::splat(1.0) / t81;
            let t84 = f64x8::splat(1.0) / t21;
            let t85 = t2 * t84;
            let t86 = t85 * t7;
            let t87 = t9 * t8;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = f64x8::splat(1.0) / t8;
            let t92 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t15 + f64x8::splat(0.0123825) * t12;
            let t95 = f64x8::splat(1.0) + t15 * t92 / f64x8::splat(2.0);
            let t96 = t95 * t95;
            let t97 = f64x8::splat(1.0) / t96;
            let t102 = t2 * t4 * t3;
            let t103 = t7 * t88;
            let t104 = t102 * t103;
            let t107 = t20 * t21 * t3;
            let t109 = f64x8::splat(1.0) / t23 / t8;
            let t110 = t6 * t109;
            let t111 = t107 * t110;
            let t113 = f64x8::splat(1.0) / t64;
            let t115 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t118 = t2 * t4 / t115;
            let t119 = t7 * t66;
            let t120 = t118 * t119;
            let t122 = -f64x8::splat(0.005977859662531589) * t89 + f64x8::splat(0.001317375) * t104 - f64x8::splat(0.00023775) * t111 + f64x8::splat(6.474423634745383e-06) * t113 - f64x8::splat(5.40140625e-07) * t120;
            let t124 = f64x8::splat(0.0011713266981940448) * t89 * t97 - t60 * t122;
            let t125 = t88 * t124;
            let t126 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t127 = t85 * t126;
            let t128 = t15 * t12;
            let t129 = t23 * t128;
            let t130 = f64x8::splat(1.0) / t95;
            let t134 = t60 * t60;
            let t136 = f64x8::splat(0.0019711289) * t127 * t129 * t130 - f64x8::splat(2.0) * t134;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t137 * t62;
            let t140 = t86 * t125 * t138;
            let t142 = t60 * t82 + f64x8::splat(0.009949166666666667) * t140;
            let t143 = ((f64x8::splat(4.0)).sqrt());
            let t144 = t60 * t143;
            let t145 = t128 * t130;
            let t148 = t7 * t23;
            let t152 = f64x8::splat(0.00619125) * t144 * t145 - f64x8::splat(0.07959333333333334) * t85 * t148 * t122;
            let t153 = t152 * t137;
            let t154 = t62 * t113;
            let t155 = t153 * t154;
            let t157 = t124 * t137;
            let t158 = t62 * t62;
            let t159 = t64 * t64;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t158 * t160;
            let t162 = t157 * t161;
            let t164 = f64x8::splat(1.0) + t155 / f64x8::splat(8.0) - t162 / f64x8::splat(64.0);
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t142 * t165;
            let t167 = v_rho0 - v_rho1;
            let t168 = t167 * t89;
            let t169 = f64x8::splat(1.0) + t168;
            let t170 = (t169).simd_le(zeta_threshold);
            let t171 = (simd::cbrt(t169));
            let t173 = ((t170).select(t37, t171 * t169));
            let t174 = f64x8::splat(1.0) - t168;
            let t175 = (t174).simd_le(zeta_threshold);
            let t176 = (simd::cbrt(t174));
            let t178 = ((t175).select(t37, t176 * t174));
            let t180 = (t173 + t178 - f64x8::splat(2.0)) * t44;
            let t183 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(2.0) * t41));
            let t185 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(0.0)));
            let t187 = (t183 + t185 - f64x8::splat(2.0)) * t44;
            let t189 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t194 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t197 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t194;
            let t198 = (simd::ln(t197));
            let t206 = -t34 + t187 * (-f64x8::splat(0.03109) * t189 * t198 + t34 - f64x8::splat(0.019751789702565206) * t57) + f64x8::splat(0.019751789702565206) * t187 * t57;
            let t207 = t62 * t66;
            let t208 = t207 * t20;
            let t209 = t68 * t6;
            let t210 = (f64x8::splat(0.0)).simd_lt(t206);
            let t212 = ((t210).select(t206, -t206));
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t209 * t213;
            let t217 = f64x8::splat(1.0) + t208 * t214 / f64x8::splat(96.0);
            let t218 = (simd::ln(t217));
            let t220 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t218;
            let t221 = f64x8::splat(1.0) / t220;
            let t224 = t206 * t221 + f64x8::splat(0.0069644166666666665) * t140;
            let t227 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t155 - f64x8::splat(0.04046875) * t162;
            let t228 = f64x8::splat(1.0) / t227;
            let t230 = t224 * t228 - t166;
            let t231 = t180 * t230;
            let t232 = f64x8::splat(1.0) / v_rho0;
            let t233 = v_sigma0 * t232;
            let t234 = f64x8::splat(1.0) / v_tau0;
            let t235 = ((t170).select(zeta_threshold, t169));
            let t236 = t234 * t235;
            let t237 = t5 * t7;
            let t238 = t10 * t41;
            let t239 = f64x8::splat(1.0) / t169;
            let t240 = (simd::cbrt(t239));
            let t242 = t237 * t238 * t240;
            let t244 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t242;
            let t245 = ((t242).sqrt());
            let t248 = ((t242) * (t242).sqrt());
            let t250 = t22 * t6;
            let t251 = t41 * t41;
            let t252 = t24 * t251;
            let t253 = t240 * t240;
            let t255 = t250 * t252 * t253;
            let t257 = f64x8::splat(3.79785) * t245 + f64x8::splat(0.8969) * t242 + f64x8::splat(0.204775) * t248 + f64x8::splat(0.123235) * t255;
            let t260 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t257;
            let t261 = (simd::ln(t260));
            let t263 = f64x8::splat(0.062182) * t244 * t261;
            let t265 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t242;
            let t270 = f64x8::splat(5.1785) * t245 + f64x8::splat(0.905775) * t242 + f64x8::splat(0.1100325) * t248 + f64x8::splat(0.1241775) * t255;
            let t273 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t270;
            let t274 = (simd::ln(t273));
            let t275 = t265 * t274;
            let t278 = -t263 + f64x8::splat(0.019751789702565206) * t45 * t275;
            let t279 = v_rho0 * v_rho0;
            let t280 = (simd::cbrt(v_rho0));
            let t281 = t280 * t280;
            let t283 = f64x8::splat(1.0) / t281 / t279;
            let t284 = v_sigma0 * t283;
            let t285 = t284 * t69;
            let t286 = t6 * t9;
            let t287 = f64x8::splat(1.0) / t240;
            let t288 = (f64x8::splat(0.0)).simd_lt(t278);
            let t290 = ((t288).select(t278, -t278));
            let t291 = f64x8::splat(1.0) / t290;
            let t292 = t287 * t291;
            let t293 = t286 * t292;
            let t296 = f64x8::splat(1.0) + t285 * t293 / f64x8::splat(96.0);
            let t297 = (simd::ln(t296));
            let t299 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t297;
            let t300 = f64x8::splat(1.0) / t299;
            let t303 = t85 * t148 * t251;
            let t304 = f64x8::splat(1.0) / t253;
            let t305 = t89 * t239;
            let t308 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t245 + f64x8::splat(0.0123825) * t242;
            let t311 = f64x8::splat(1.0) + t245 * t308 / f64x8::splat(2.0);
            let t312 = t311 * t311;
            let t313 = f64x8::splat(1.0) / t312;
            let t317 = t102 * t7;
            let t318 = t88 * t41;
            let t319 = t240 * t239;
            let t323 = t107 * t6;
            let t324 = t109 * t251;
            let t325 = t253 * t239;
            let t329 = t169 * t169;
            let t330 = f64x8::splat(1.0) / t329;
            let t331 = t113 * t330;
            let t333 = t118 * t7;
            let t334 = t66 * t41;
            let t335 = t240 * t330;
            let t339 = -f64x8::splat(0.011955719325063178) * t305 + f64x8::splat(0.00263475) * t317 * t318 * t319 - f64x8::splat(0.0004755) * t323 * t324 * t325 + f64x8::splat(2.5897694538981533e-05) * t331 - f64x8::splat(2.1605625e-06) * t333 * t334 * t335;
            let t341 = f64x8::splat(0.0023426533963880895) * t305 * t313 - t278 * t339;
            let t342 = t304 * t341;
            let t343 = t126 * t23;
            let t344 = t85 * t343;
            let t345 = t41 * t304;
            let t346 = t245 * t242;
            let t347 = f64x8::splat(1.0) / t311;
            let t348 = t346 * t347;
            let t349 = t345 * t348;
            let t352 = t278 * t278;
            let t354 = f64x8::splat(0.00098556445) * t344 * t349 - f64x8::splat(2.0) * t352;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t342 * t355;
            let t357 = t8 * t169;
            let t358 = (simd::cbrt(t357));
            let t359 = t358 * t358;
            let t360 = t284 * t359;
            let t361 = t356 * t360;
            let t362 = t303 * t361;
            let t364 = t278 * t300 + f64x8::splat(0.0024872916666666667) * t362;
            let t365 = t278 * t143;
            let t368 = t23 * t41;
            let t369 = t304 * t339;
            let t373 = f64x8::splat(0.00619125) * t365 * t348 - f64x8::splat(0.03979666666666667) * t86 * t368 * t369;
            let t374 = t373 * t355;
            let t375 = t374 * v_sigma0;
            let t376 = t283 * t41;
            let t377 = t376 * t359;
            let t378 = t375 * t377;
            let t380 = t341 * t355;
            let t381 = v_sigma0 * v_sigma0;
            let t382 = t380 * t381;
            let t383 = t279 * t279;
            let t384 = t383 * v_rho0;
            let t386 = f64x8::splat(1.0) / t280 / t384;
            let t387 = t386 * t251;
            let t388 = t358 * t357;
            let t389 = t387 * t388;
            let t390 = t382 * t389;
            let t392 = f64x8::splat(1.0) + t378 / f64x8::splat(16.0) - t390 / f64x8::splat(256.0);
            let t393 = f64x8::splat(1.0) / t392;
            let t394 = t364 * t393;
            let t396 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t242;
            let t401 = f64x8::splat(7.05945) * t245 + f64x8::splat(1.549425) * t242 + f64x8::splat(0.420775) * t248 + f64x8::splat(0.1562925) * t255;
            let t404 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t401;
            let t405 = (simd::ln(t404));
            let t413 = -t263 + t187 * (-f64x8::splat(0.03109) * t396 * t405 + t263 - f64x8::splat(0.019751789702565206) * t275) + f64x8::splat(0.019751789702565206) * t187 * t275;
            let t414 = t251 * v_sigma0;
            let t415 = t283 * t20;
            let t416 = t414 * t415;
            let t417 = t9 * t287;
            let t418 = (f64x8::splat(0.0)).simd_lt(t413);
            let t420 = ((t418).select(t413, -t413));
            let t421 = f64x8::splat(1.0) / t420;
            let t423 = t209 * t417 * t421;
            let t426 = f64x8::splat(1.0) + t416 * t423 / f64x8::splat(192.0);
            let t427 = (simd::ln(t426));
            let t429 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t427;
            let t430 = f64x8::splat(1.0) / t429;
            let t433 = t413 * t430 + f64x8::splat(0.0017411041666666666) * t362;
            let t436 = f64x8::splat(1.0) + f64x8::splat(0.09375) * t378 - f64x8::splat(0.0101171875) * t390;
            let t437 = f64x8::splat(1.0) / t436;
            let t441 = t394 + t187 * (t433 * t437 - t394);
            let t442 = t236 * t441;
            let t444 = t233 * t442 / f64x8::splat(16.0);
            let t445 = f64x8::splat(1.0) / v_rho1;
            let t446 = v_sigma2 * t445;
            let t447 = f64x8::splat(1.0) / v_tau1;
            let t448 = ((t175).select(zeta_threshold, t174));
            let t449 = t447 * t448;
            let t450 = f64x8::splat(1.0) / t174;
            let t451 = (simd::cbrt(t450));
            let t453 = t237 * t238 * t451;
            let t455 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t453;
            let t456 = ((t453).sqrt());
            let t459 = ((t453) * (t453).sqrt());
            let t461 = t451 * t451;
            let t463 = t250 * t252 * t461;
            let t465 = f64x8::splat(3.79785) * t456 + f64x8::splat(0.8969) * t453 + f64x8::splat(0.204775) * t459 + f64x8::splat(0.123235) * t463;
            let t468 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t465;
            let t469 = (simd::ln(t468));
            let t471 = f64x8::splat(0.062182) * t455 * t469;
            let t473 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t453;
            let t478 = f64x8::splat(5.1785) * t456 + f64x8::splat(0.905775) * t453 + f64x8::splat(0.1100325) * t459 + f64x8::splat(0.1241775) * t463;
            let t481 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t478;
            let t482 = (simd::ln(t481));
            let t483 = t473 * t482;
            let t486 = -t471 + f64x8::splat(0.019751789702565206) * t45 * t483;
            let t487 = v_rho1 * v_rho1;
            let t488 = (simd::cbrt(v_rho1));
            let t489 = t488 * t488;
            let t491 = f64x8::splat(1.0) / t489 / t487;
            let t492 = v_sigma2 * t491;
            let t493 = t492 * t69;
            let t494 = f64x8::splat(1.0) / t451;
            let t495 = (f64x8::splat(0.0)).simd_lt(t486);
            let t497 = ((t495).select(t486, -t486));
            let t498 = f64x8::splat(1.0) / t497;
            let t499 = t494 * t498;
            let t500 = t286 * t499;
            let t503 = f64x8::splat(1.0) + t493 * t500 / f64x8::splat(96.0);
            let t504 = (simd::ln(t503));
            let t506 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t504;
            let t507 = f64x8::splat(1.0) / t506;
            let t509 = f64x8::splat(1.0) / t461;
            let t510 = t89 * t450;
            let t513 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t456 + f64x8::splat(0.0123825) * t453;
            let t516 = f64x8::splat(1.0) + t456 * t513 / f64x8::splat(2.0);
            let t517 = t516 * t516;
            let t518 = f64x8::splat(1.0) / t517;
            let t522 = t451 * t450;
            let t526 = t461 * t450;
            let t530 = t174 * t174;
            let t531 = f64x8::splat(1.0) / t530;
            let t532 = t113 * t531;
            let t534 = t451 * t531;
            let t538 = -f64x8::splat(0.011955719325063178) * t510 + f64x8::splat(0.00263475) * t317 * t318 * t522 - f64x8::splat(0.0004755) * t323 * t324 * t526 + f64x8::splat(2.5897694538981533e-05) * t532 - f64x8::splat(2.1605625e-06) * t333 * t334 * t534;
            let t540 = f64x8::splat(0.0023426533963880895) * t510 * t518 - t486 * t538;
            let t541 = t509 * t540;
            let t542 = t41 * t509;
            let t543 = t456 * t453;
            let t544 = f64x8::splat(1.0) / t516;
            let t545 = t543 * t544;
            let t546 = t542 * t545;
            let t549 = t486 * t486;
            let t551 = f64x8::splat(0.00098556445) * t344 * t546 - f64x8::splat(2.0) * t549;
            let t552 = f64x8::splat(1.0) / t551;
            let t553 = t541 * t552;
            let t554 = t8 * t174;
            let t555 = (simd::cbrt(t554));
            let t556 = t555 * t555;
            let t557 = t492 * t556;
            let t558 = t553 * t557;
            let t559 = t303 * t558;
            let t561 = t486 * t507 + f64x8::splat(0.0024872916666666667) * t559;
            let t562 = t486 * t143;
            let t565 = t509 * t538;
            let t569 = f64x8::splat(0.00619125) * t562 * t545 - f64x8::splat(0.03979666666666667) * t86 * t368 * t565;
            let t570 = t569 * t552;
            let t571 = t570 * v_sigma2;
            let t572 = t491 * t41;
            let t573 = t572 * t556;
            let t574 = t571 * t573;
            let t576 = t540 * t552;
            let t577 = v_sigma2 * v_sigma2;
            let t578 = t576 * t577;
            let t579 = t487 * t487;
            let t580 = t579 * v_rho1;
            let t582 = f64x8::splat(1.0) / t488 / t580;
            let t583 = t582 * t251;
            let t584 = t555 * t554;
            let t585 = t583 * t584;
            let t586 = t578 * t585;
            let t588 = f64x8::splat(1.0) + t574 / f64x8::splat(16.0) - t586 / f64x8::splat(256.0);
            let t589 = f64x8::splat(1.0) / t588;
            let t590 = t561 * t589;
            let t592 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t453;
            let t597 = f64x8::splat(7.05945) * t456 + f64x8::splat(1.549425) * t453 + f64x8::splat(0.420775) * t459 + f64x8::splat(0.1562925) * t463;
            let t600 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t597;
            let t601 = (simd::ln(t600));
            let t609 = -t471 + t187 * (-f64x8::splat(0.03109) * t592 * t601 + t471 - f64x8::splat(0.019751789702565206) * t483) + f64x8::splat(0.019751789702565206) * t187 * t483;
            let t610 = t251 * v_sigma2;
            let t611 = t491 * t20;
            let t612 = t610 * t611;
            let t613 = t9 * t494;
            let t614 = (f64x8::splat(0.0)).simd_lt(t609);
            let t616 = ((t614).select(t609, -t609));
            let t617 = f64x8::splat(1.0) / t616;
            let t619 = t209 * t613 * t617;
            let t622 = f64x8::splat(1.0) + t612 * t619 / f64x8::splat(192.0);
            let t623 = (simd::ln(t622));
            let t625 = f64x8::splat(1.0) + f64x8::splat(0.066725) * t623;
            let t626 = f64x8::splat(1.0) / t625;
            let t629 = t609 * t626 + f64x8::splat(0.0017411041666666666) * t559;
            let t632 = f64x8::splat(1.0) + f64x8::splat(0.09375) * t574 - f64x8::splat(0.0101171875) * t586;
            let t633 = f64x8::splat(1.0) / t632;
            let t637 = t590 + t187 * (t629 * t633 - t590);
            let t638 = t449 * t637;
            let t640 = t446 * t638 / f64x8::splat(16.0);
            let tzk0 = t166 + t231 - t444 - t640;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
