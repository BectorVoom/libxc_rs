//! LDA_C_VWN_3 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_3.c`
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
pub fn lda_c_vwn_3_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
            let t34 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t35 = f64x8::splat(1.0) / t34;
            let t39 = (simd::ln(t4 * t10 * t35 / f64x8::splat(4.0)));
            let t41 = t13 + f64x8::splat(7.06042);
            let t44 = (simd::atan(f64x8::splat(4.730926909560113) / t41));
            let t46 = t27 + f64x8::splat(0.325);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t35));
            let t51 = f64x8::splat(0.01554535) * t39 + f64x8::splat(0.05249139316978094) * t44 + f64x8::splat(0.0022478670955426118) * t49 - t21 - t26 - t32;
            let t53 = t12 + f64x8::splat(10.06155) * t13 + f64x8::splat(101.578);
            let t54 = f64x8::splat(1.0) / t53;
            let t58 = (simd::ln(t4 * t10 * t54 / f64x8::splat(4.0)));
            let t60 = t13 + f64x8::splat(20.1231);
            let t63 = (simd::atan(f64x8::splat(1.171685277708993) / t60));
            let t65 = t27 + f64x8::splat(0.743294);
            let t66 = t65 * t65;
            let t68 = (simd::ln(t66 * t54));
            let t71 = t12 + f64x8::splat(6.536) * t13 + f64x8::splat(42.7198);
            let t72 = f64x8::splat(1.0) / t71;
            let t76 = (simd::ln(t4 * t10 * t72 / f64x8::splat(4.0)));
            let t78 = t13 + f64x8::splat(13.072);
            let t81 = (simd::atan(f64x8::splat(0.0448998886412873) / t78));
            let t83 = t27 + f64x8::splat(0.409286);
            let t84 = t83 * t83;
            let t86 = (simd::ln(t84 * t72));
            let t88 = f64x8::splat(0.01554535) * t58 + f64x8::splat(0.6188180297906063) * t63 + f64x8::splat(0.002667310007273315) * t68 - f64x8::splat(0.0310907) * t76 - f64x8::splat(20.521972937837504) * t81 - f64x8::splat(0.004431373767749538) * t86;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t51 * t89;
            let t91 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = t12 + f64x8::splat(0.534175) * t13 + f64x8::splat(11.4813);
            let t95 = f64x8::splat(1.0) / t94;
            let t99 = (simd::ln(t4 * t10 * t95 / f64x8::splat(4.0)));
            let t100 = t13 + f64x8::splat(1.06835);
            let t103 = (simd::atan(f64x8::splat(6.692072046645942) / t100));
            let t105 = t27 + f64x8::splat(0.228344);
            let t106 = t105 * t105;
            let t108 = (simd::ln(t106 * t95));
            let t111 = t92 * (t99 + f64x8::splat(0.32323836906055065) * t103 + f64x8::splat(0.021608710360898266) * t108);
            let t112 = t90 * t111;
            let t113 = v_rho0 - v_rho1;
            let t114 = f64x8::splat(1.0) / t7;
            let t115 = t113 * t114;
            let t116 = f64x8::splat(1.0) + t115;
            let t117 = (t116).simd_le(zeta_threshold);
            let t118 = (simd::cbrt(zeta_threshold));
            let t119 = t118 * zeta_threshold;
            let t120 = (simd::cbrt(t116));
            let t122 = ((t117).select(t119, t120 * t116));
            let t123 = f64x8::splat(1.0) - t115;
            let t124 = (t123).simd_le(zeta_threshold);
            let t125 = (simd::cbrt(t123));
            let t127 = ((t124).select(t119, t125 * t123));
            let t128 = t122 + t127 - f64x8::splat(2.0);
            let t129 = f64x8::splat(M_CBRT2);
            let t130 = t129 - f64x8::splat(1.0);
            let t132 = f64x8::splat(1.0) / t130 / f64x8::splat(2.0);
            let t133 = t128 * t132;
            let t134 = t113 * t113;
            let t135 = t134 * t134;
            let t136 = t7 * t7;
            let t137 = t136 * t136;
            let t138 = f64x8::splat(1.0) / t137;
            let t140 = -t135 * t138 + f64x8::splat(1.0);
            let t141 = f64x8::splat(9.0) * t130;
            let t142 = t140 * t141;
            let t143 = t133 * t142;
            let t145 = t112 * t143 / f64x8::splat(24.0);
            let t146 = t51 * t128;
            let t147 = t132 * t135;
            let t148 = t147 * t138;
            let t149 = t146 * t148;
            let tzk0 = t21 + t26 + t32 - t145 + t149;
            acc_zk = tzk0;
            let t151 = f64x8::splat(1.0) / t8 / t7;
            let t152 = t6 * t151;
            let t156 = t4 * t6;
            let t157 = t15 * t15;
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t9 * t158;
            let t160 = t4 * t152;
            let t161 = t160 / f64x8::splat(12.0);
            let t162 = f64x8::splat(1.0) / t13;
            let t163 = t162 * t1;
            let t164 = t3 * t6;
            let t166 = t163 * t164 * t151;
            let t168 = -t161 - f64x8::splat(0.31062) * t166;
            let t173 = t1 * t1;
            let t175 = f64x8::splat(1.0) / t3;
            let t176 = (-t4 * t152 * t16 / f64x8::splat(12.0) - t156 * t159 * t168 / f64x8::splat(4.0)) * t173 * t175;
            let t177 = t5 * t8;
            let t178 = t177 * t15;
            let t179 = t176 * t178;
            let t180 = f64x8::splat(0.010363566666666667) * t179;
            let t181 = t22 * t22;
            let t182 = f64x8::splat(1.0) / t181;
            let t184 = t182 * t162 * t1;
            let t186 = f64x8::splat(37.8469910464) * t182 + f64x8::splat(1.0);
            let t187 = f64x8::splat(1.0) / t186;
            let t190 = t184 * t164 * t151 * t187;
            let t191 = f64x8::splat(0.03976574567502677) * t190;
            let t192 = t28 * t16;
            let t193 = t192 * t162;
            let t196 = t29 * t158;
            let t198 = -t193 * t160 / f64x8::splat(6.0) - t196 * t168;
            let t199 = f64x8::splat(1.0) / t29;
            let t200 = t198 * t199;
            let t201 = t200 * t15;
            let t202 = f64x8::splat(0.0009690227711544374) * t201;
            let t206 = t34 * t34;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t9 * t207;
            let t210 = -t161 - f64x8::splat(0.5883683333333334) * t166;
            let t216 = (-t4 * t152 * t35 / f64x8::splat(12.0) - t156 * t208 * t210 / f64x8::splat(4.0)) * t173 * t175;
            let t217 = t177 * t34;
            let t220 = t41 * t41;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = t221 * t162 * t1;
            let t225 = f64x8::splat(22.3816694236) * t221 + f64x8::splat(1.0);
            let t226 = f64x8::splat(1.0) / t225;
            let t231 = t46 * t35;
            let t232 = t231 * t162;
            let t235 = t47 * t207;
            let t237 = -t232 * t160 / f64x8::splat(6.0) - t235 * t210;
            let t238 = f64x8::splat(1.0) / t47;
            let t239 = t237 * t238;
            let t242 = f64x8::splat(0.005181783333333334) * t216 * t217 + f64x8::splat(0.041388824077869424) * t223 * t164 * t151 * t226 + f64x8::splat(0.0022478670955426118) * t239 * t34 - t180 - t191 - t202;
            let t243 = t242 * t89;
            let t244 = t243 * t111;
            let t245 = t244 * t143;
            let t246 = t245 / f64x8::splat(24.0);
            let t247 = t88 * t88;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t51 * t248;
            let t250 = t249 * t111;
            let t254 = t53 * t53;
            let t255 = f64x8::splat(1.0) / t254;
            let t256 = t9 * t255;
            let t258 = -t161 - f64x8::splat(1.676925) * t166;
            let t264 = (-t4 * t152 * t54 / f64x8::splat(12.0) - t156 * t256 * t258 / f64x8::splat(4.0)) * t173 * t175;
            let t265 = t177 * t53;
            let t268 = t60 * t60;
            let t269 = f64x8::splat(1.0) / t268;
            let t271 = t269 * t162 * t1;
            let t273 = f64x8::splat(1.37284639) * t269 + f64x8::splat(1.0);
            let t274 = f64x8::splat(1.0) / t273;
            let t279 = t65 * t54;
            let t280 = t279 * t162;
            let t283 = t66 * t255;
            let t285 = -t280 * t160 / f64x8::splat(6.0) - t283 * t258;
            let t286 = f64x8::splat(1.0) / t66;
            let t287 = t285 * t286;
            let t293 = t71 * t71;
            let t294 = f64x8::splat(1.0) / t293;
            let t295 = t9 * t294;
            let t297 = -t161 - f64x8::splat(1.0893333333333333) * t166;
            let t303 = (-t4 * t152 * t72 / f64x8::splat(12.0) - t156 * t295 * t297 / f64x8::splat(4.0)) * t173 * t175;
            let t304 = t177 * t71;
            let t307 = t78 * t78;
            let t308 = f64x8::splat(1.0) / t307;
            let t310 = t308 * t162 * t1;
            let t312 = f64x8::splat(0.002016) * t308 + f64x8::splat(1.0);
            let t313 = f64x8::splat(1.0) / t312;
            let t318 = t83 * t72;
            let t319 = t318 * t162;
            let t322 = t84 * t294;
            let t324 = -t319 * t160 / f64x8::splat(6.0) - t322 * t297;
            let t325 = f64x8::splat(1.0) / t84;
            let t326 = t324 * t325;
            let t329 = f64x8::splat(0.005181783333333334) * t264 * t265 + f64x8::splat(0.12084332918108974) * t271 * t164 * t151 * t274 + f64x8::splat(0.002667310007273315) * t287 * t53 - f64x8::splat(0.010363566666666667) * t303 * t304 - f64x8::splat(0.15357238326806924) * t310 * t164 * t151 * t313 - f64x8::splat(0.004431373767749538) * t326 * t71;
            let t330 = t142 * t329;
            let t331 = t133 * t330;
            let t332 = t250 * t331;
            let t333 = t332 / f64x8::splat(24.0);
            let t337 = t94 * t94;
            let t338 = f64x8::splat(1.0) / t337;
            let t339 = t9 * t338;
            let t341 = -t161 - f64x8::splat(0.08902916666666667) * t166;
            let t347 = (-t4 * t152 * t95 / f64x8::splat(12.0) - t156 * t339 * t341 / f64x8::splat(4.0)) * t173 * t175;
            let t348 = t177 * t94;
            let t351 = t100 * t100;
            let t352 = f64x8::splat(1.0) / t351;
            let t354 = t352 * t162 * t1;
            let t356 = f64x8::splat(44.7838282775) * t352 + f64x8::splat(1.0);
            let t357 = f64x8::splat(1.0) / t356;
            let t362 = t105 * t95;
            let t363 = t362 * t162;
            let t366 = t106 * t338;
            let t368 = -t363 * t160 / f64x8::splat(6.0) - t366 * t341;
            let t369 = f64x8::splat(1.0) / t106;
            let t370 = t368 * t369;
            let t374 = t92 * (t347 * t348 / f64x8::splat(3.0) + f64x8::splat(0.36052240899892257) * t354 * t164 * t151 * t357 + f64x8::splat(0.021608710360898266) * t370 * t94);
            let t375 = t90 * t374;
            let t376 = t375 * t143;
            let t377 = t376 / f64x8::splat(24.0);
            let t378 = f64x8::splat(1.0) / t136;
            let t379 = t113 * t378;
            let t380 = t114 - t379;
            let t383 = ((t117).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t120 * t380));
            let t384 = -t380;
            let t387 = ((t124).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t125 * t384));
            let t388 = t383 + t387;
            let t389 = t388 * t132;
            let t390 = t389 * t142;
            let t391 = t112 * t390;
            let t392 = t391 / f64x8::splat(24.0);
            let t393 = t134 * t113;
            let t394 = t393 * t138;
            let t395 = t137 * t7;
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t135 * t396;
            let t399 = -f64x8::splat(4.0) * t394 + f64x8::splat(4.0) * t397;
            let t400 = t399 * t141;
            let t401 = t133 * t400;
            let t402 = t112 * t401;
            let t403 = t402 / f64x8::splat(24.0);
            let t404 = t242 * t128;
            let t405 = t404 * t148;
            let t406 = t51 * t388;
            let t407 = t406 * t148;
            let t408 = t132 * t393;
            let t409 = t408 * t138;
            let t410 = t146 * t409;
            let t411 = f64x8::splat(4.0) * t410;
            let t412 = t147 * t396;
            let t413 = t146 * t412;
            let t414 = f64x8::splat(4.0) * t413;
            let t415 = t180 + t191 + t202 - t246 + t333 - t377 - t392 - t403 + t405 + t407 + t411 - t414;
            let tvrho0 = t7 * t415 - t145 + t149 + t21 + t26 + t32;
            acc_vrho_0 = tvrho0;
            let t417 = -t114 - t379;
            let t420 = ((t117).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t120 * t417));
            let t421 = -t417;
            let t424 = ((t124).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t125 * t421));
            let t425 = t420 + t424;
            let t426 = t425 * t132;
            let t427 = t426 * t142;
            let t428 = t112 * t427;
            let t429 = t428 / f64x8::splat(24.0);
            let t431 = f64x8::splat(4.0) * t394 + f64x8::splat(4.0) * t397;
            let t432 = t431 * t141;
            let t433 = t133 * t432;
            let t434 = t112 * t433;
            let t435 = t434 / f64x8::splat(24.0);
            let t436 = t51 * t425;
            let t437 = t436 * t148;
            let t438 = t180 + t191 + t202 - t246 + t333 - t377 - t429 - t435 + t405 + t437 - t411 - t414;
            let tvrho1 = t7 * t438 - t145 + t149 + t21 + t26 + t32;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
