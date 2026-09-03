//! LDA_C_VWN_2 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_2.c`
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
pub fn lda_c_vwn_2_vxc_pol(
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
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = f64x8::splat(1.0) / t33;
            let t36 = t12 + f64x8::splat(0.534175) * t13 + f64x8::splat(11.4813);
            let t37 = f64x8::splat(1.0) / t36;
            let t41 = (simd::ln(t4 * t10 * t37 / f64x8::splat(4.0)));
            let t42 = t13 + f64x8::splat(1.06835);
            let t45 = (simd::atan(f64x8::splat(6.692072046645942) / t42));
            let t47 = t27 + f64x8::splat(0.228344);
            let t48 = t47 * t47;
            let t50 = (simd::ln(t48 * t37));
            let t53 = t34 * (t41 + f64x8::splat(0.32323836906055065) * t45 + f64x8::splat(0.021608710360898266) * t50);
            let t54 = v_rho0 - v_rho1;
            let t55 = f64x8::splat(1.0) / t7;
            let t56 = t54 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t63 = ((t58).select(t60, t61 * t57));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t60, t66 * t64));
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
            let t82 = t74 * (-t76 * t79 + f64x8::splat(1.0));
            let t83 = f64x8::splat(9.0) * t72;
            let t84 = t82 * t83;
            let t86 = t70 * t84 / f64x8::splat(24.0);
            let t88 = t12 + f64x8::splat(10.06155) * t13 + f64x8::splat(101.578);
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = (simd::ln(t4 * t10 * t89 / f64x8::splat(4.0)));
            let t95 = t13 + f64x8::splat(20.1231);
            let t98 = (simd::atan(f64x8::splat(1.171685277708993) / t95));
            let t100 = t27 + f64x8::splat(0.743294);
            let t101 = t100 * t100;
            let t103 = (simd::ln(t101 * t89));
            let t106 = t12 + f64x8::splat(6.536) * t13 + f64x8::splat(42.7198);
            let t107 = f64x8::splat(1.0) / t106;
            let t111 = (simd::ln(t4 * t10 * t107 / f64x8::splat(4.0)));
            let t113 = t13 + f64x8::splat(13.072);
            let t116 = (simd::atan(f64x8::splat(0.0448998886412873) / t113));
            let t118 = t27 + f64x8::splat(0.409286);
            let t119 = t118 * t118;
            let t121 = (simd::ln(t119 * t107));
            let t123 = f64x8::splat(0.01554535) * t93 + f64x8::splat(0.6188180297906063) * t98 + f64x8::splat(0.002667310007273315) * t103 - f64x8::splat(0.0310907) * t111 - f64x8::splat(20.521972937837504) * t116 - f64x8::splat(0.004431373767749538) * t121;
            let t124 = t123 * t69;
            let t125 = t124 * t82;
            let t127 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t128 = f64x8::splat(1.0) / t127;
            let t132 = (simd::ln(t4 * t10 * t128 / f64x8::splat(4.0)));
            let t134 = t13 + f64x8::splat(7.06042);
            let t137 = (simd::atan(f64x8::splat(4.730926909560113) / t134));
            let t139 = t27 + f64x8::splat(0.325);
            let t140 = t139 * t139;
            let t142 = (simd::ln(t140 * t128));
            let t144 = f64x8::splat(0.01554535) * t132 + f64x8::splat(0.05249139316978094) * t137 + f64x8::splat(0.0022478670955426118) * t142 - t21 - t26 - t32;
            let t146 = t144 * t69 * t74;
            let tzk0 = t21 + t26 + t32 - t86 - t125 + t146;
            acc_zk = tzk0;
            let t148 = f64x8::splat(1.0) / t8 / t7;
            let t149 = t6 * t148;
            let t153 = t4 * t6;
            let t154 = t15 * t15;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t9 * t155;
            let t157 = t4 * t149;
            let t158 = t157 / f64x8::splat(12.0);
            let t159 = f64x8::splat(1.0) / t13;
            let t160 = t159 * t1;
            let t161 = t3 * t6;
            let t163 = t160 * t161 * t148;
            let t165 = -t158 - f64x8::splat(0.31062) * t163;
            let t170 = t1 * t1;
            let t172 = f64x8::splat(1.0) / t3;
            let t173 = (-t4 * t149 * t16 / f64x8::splat(12.0) - t153 * t156 * t165 / f64x8::splat(4.0)) * t170 * t172;
            let t174 = t5 * t8;
            let t175 = t174 * t15;
            let t176 = t173 * t175;
            let t177 = f64x8::splat(0.010363566666666667) * t176;
            let t178 = t22 * t22;
            let t179 = f64x8::splat(1.0) / t178;
            let t181 = t179 * t159 * t1;
            let t183 = f64x8::splat(37.8469910464) * t179 + f64x8::splat(1.0);
            let t184 = f64x8::splat(1.0) / t183;
            let t187 = t181 * t161 * t148 * t184;
            let t188 = f64x8::splat(0.03976574567502677) * t187;
            let t189 = t28 * t16;
            let t190 = t189 * t159;
            let t193 = t29 * t155;
            let t195 = -t190 * t157 / f64x8::splat(6.0) - t193 * t165;
            let t196 = f64x8::splat(1.0) / t29;
            let t197 = t195 * t196;
            let t198 = t197 * t15;
            let t199 = f64x8::splat(0.0009690227711544374) * t198;
            let t203 = t36 * t36;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t9 * t204;
            let t207 = -t158 - f64x8::splat(0.08902916666666667) * t163;
            let t213 = (-t4 * t149 * t37 / f64x8::splat(12.0) - t153 * t205 * t207 / f64x8::splat(4.0)) * t170 * t172;
            let t214 = t174 * t36;
            let t217 = t42 * t42;
            let t218 = f64x8::splat(1.0) / t217;
            let t220 = t218 * t159 * t1;
            let t222 = f64x8::splat(44.7838282775) * t218 + f64x8::splat(1.0);
            let t223 = f64x8::splat(1.0) / t222;
            let t228 = t47 * t37;
            let t229 = t228 * t159;
            let t232 = t48 * t204;
            let t234 = -t229 * t157 / f64x8::splat(6.0) - t232 * t207;
            let t235 = f64x8::splat(1.0) / t48;
            let t236 = t234 * t235;
            let t240 = t34 * (t213 * t214 / f64x8::splat(3.0) + f64x8::splat(0.36052240899892257) * t220 * t161 * t148 * t223 + f64x8::splat(0.021608710360898266) * t236 * t36);
            let t241 = t240 * t69;
            let t242 = t241 * t84;
            let t243 = t242 / f64x8::splat(24.0);
            let t244 = f64x8::splat(1.0) / t77;
            let t245 = t54 * t244;
            let t246 = t55 - t245;
            let t249 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t246));
            let t250 = -t246;
            let t253 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t250));
            let t254 = t249 + t253;
            let t255 = t53 * t254;
            let t256 = t255 * t84;
            let t257 = t256 / f64x8::splat(24.0);
            let t258 = t75 * t54;
            let t259 = t258 * t79;
            let t260 = t78 * t7;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t76 * t261;
            let t265 = t74 * (-f64x8::splat(4.0) * t259 + f64x8::splat(4.0) * t262);
            let t266 = t265 * t83;
            let t267 = t70 * t266;
            let t268 = t267 / f64x8::splat(24.0);
            let t272 = t88 * t88;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t9 * t273;
            let t276 = -t158 - f64x8::splat(1.676925) * t163;
            let t282 = (-t4 * t149 * t89 / f64x8::splat(12.0) - t153 * t274 * t276 / f64x8::splat(4.0)) * t170 * t172;
            let t283 = t174 * t88;
            let t286 = t95 * t95;
            let t287 = f64x8::splat(1.0) / t286;
            let t289 = t287 * t159 * t1;
            let t291 = f64x8::splat(1.37284639) * t287 + f64x8::splat(1.0);
            let t292 = f64x8::splat(1.0) / t291;
            let t297 = t100 * t89;
            let t298 = t297 * t159;
            let t301 = t101 * t273;
            let t303 = -t298 * t157 / f64x8::splat(6.0) - t301 * t276;
            let t304 = f64x8::splat(1.0) / t101;
            let t305 = t303 * t304;
            let t311 = t106 * t106;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t9 * t312;
            let t315 = -t158 - f64x8::splat(1.0893333333333333) * t163;
            let t321 = (-t4 * t149 * t107 / f64x8::splat(12.0) - t153 * t313 * t315 / f64x8::splat(4.0)) * t170 * t172;
            let t322 = t174 * t106;
            let t325 = t113 * t113;
            let t326 = f64x8::splat(1.0) / t325;
            let t328 = t326 * t159 * t1;
            let t330 = f64x8::splat(0.002016) * t326 + f64x8::splat(1.0);
            let t331 = f64x8::splat(1.0) / t330;
            let t336 = t118 * t107;
            let t337 = t336 * t159;
            let t340 = t119 * t312;
            let t342 = -t337 * t157 / f64x8::splat(6.0) - t340 * t315;
            let t343 = f64x8::splat(1.0) / t119;
            let t344 = t342 * t343;
            let t347 = f64x8::splat(0.005181783333333334) * t282 * t283 + f64x8::splat(0.12084332918108974) * t289 * t161 * t148 * t292 + f64x8::splat(0.002667310007273315) * t305 * t88 - f64x8::splat(0.010363566666666667) * t321 * t322 - f64x8::splat(0.15357238326806924) * t328 * t161 * t148 * t331 - f64x8::splat(0.004431373767749538) * t344 * t106;
            let t348 = t347 * t69;
            let t349 = t348 * t82;
            let t350 = t123 * t254;
            let t351 = t350 * t82;
            let t352 = t124 * t265;
            let t356 = t127 * t127;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t9 * t357;
            let t360 = -t158 - f64x8::splat(0.5883683333333334) * t163;
            let t366 = (-t4 * t149 * t128 / f64x8::splat(12.0) - t153 * t358 * t360 / f64x8::splat(4.0)) * t170 * t172;
            let t367 = t174 * t127;
            let t370 = t134 * t134;
            let t371 = f64x8::splat(1.0) / t370;
            let t373 = t371 * t159 * t1;
            let t375 = f64x8::splat(22.3816694236) * t371 + f64x8::splat(1.0);
            let t376 = f64x8::splat(1.0) / t375;
            let t381 = t139 * t128;
            let t382 = t381 * t159;
            let t385 = t140 * t357;
            let t387 = -t382 * t157 / f64x8::splat(6.0) - t385 * t360;
            let t388 = f64x8::splat(1.0) / t140;
            let t389 = t387 * t388;
            let t392 = f64x8::splat(0.005181783333333334) * t366 * t367 + f64x8::splat(0.041388824077869424) * t373 * t161 * t148 * t376 + f64x8::splat(0.0022478670955426118) * t389 * t127 - t177 - t188 - t199;
            let t394 = t392 * t69 * t74;
            let t396 = t144 * t254 * t74;
            let t397 = t177 + t188 + t199 - t243 - t257 - t268 - t349 - t351 - t352 + t394 + t396;
            let tvrho0 = t7 * t397 - t125 + t146 + t21 + t26 + t32 - t86;
            acc_vrho_0 = tvrho0;
            let t399 = -t55 - t245;
            let t402 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t399));
            let t403 = -t399;
            let t406 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t403));
            let t407 = t402 + t406;
            let t408 = t53 * t407;
            let t409 = t408 * t84;
            let t410 = t409 / f64x8::splat(24.0);
            let t413 = t74 * (f64x8::splat(4.0) * t259 + f64x8::splat(4.0) * t262);
            let t414 = t413 * t83;
            let t415 = t70 * t414;
            let t416 = t415 / f64x8::splat(24.0);
            let t417 = t123 * t407;
            let t418 = t417 * t82;
            let t419 = t124 * t413;
            let t421 = t144 * t407 * t74;
            let t422 = t177 + t188 + t199 - t243 - t410 - t416 - t349 - t418 - t419 + t394 + t421;
            let tvrho1 = t7 * t422 - t125 + t146 + t21 + t26 + t32 - t86;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
