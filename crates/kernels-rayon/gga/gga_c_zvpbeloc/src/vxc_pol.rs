//! GGA_C_ZVPBELOC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeloc.c`
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
pub fn gga_c_zvpbeloc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t2;
            let t5 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t7 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = (simd::pow(t8, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t10 = t3 * t1 * t5 * t9;
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = v_rho0 + v_rho1;
            let t14 = (simd::cbrt(t13));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t12 * t15;
            let t17 = v_rho0 - v_rho1;
            let t18 = t17 * t17;
            let t19 = t13 * t13;
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t18 * t20;
            let t22 = (f64x8::splat(1e-20)).simd_lt(t21);
            let t23 = ((t22).select(t21, f64x8::splat(1e-20)));
            let t27 = (simd::exp(-f64x8::splat(1.0) * t10 * t16 * t23));
            let t28 = f64x8::splat(M_CBRT3);
            let t29 = t28 * t12;
            let t30 = f64x8::splat(M_CBRT4);
            let t31 = t30 * t30;
            let t33 = t29 * t31 * t15;
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t33;
            let t36 = ((t33).sqrt());
            let t39 = ((t33) * (t33).sqrt());
            let t41 = t28 * t28;
            let t42 = t12 * t12;
            let t43 = t41 * t42;
            let t44 = t14 * t14;
            let t47 = t43 * t30 / t44;
            let t49 = f64x8::splat(3.79785) * t36 + f64x8::splat(0.8969) * t33 + f64x8::splat(0.204775) * t39 + f64x8::splat(0.123235) * t47;
            let t52 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t49;
            let t53 = (simd::ln(t52));
            let t55 = f64x8::splat(0.0621814) * t35 * t53;
            let t56 = t18 * t18;
            let t57 = t19 * t19;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t56 * t58;
            let t60 = f64x8::splat(1.0) / t13;
            let t61 = t17 * t60;
            let t62 = f64x8::splat(1.0) + t61;
            let t63 = (t62).simd_le(zeta_threshold);
            let t64 = (simd::cbrt(zeta_threshold));
            let t65 = t64 * zeta_threshold;
            let t66 = (simd::cbrt(t62));
            let t67 = t66 * t62;
            let t68 = ((t63).select(t65, t67));
            let t69 = f64x8::splat(1.0) - t61;
            let t70 = (t69).simd_le(zeta_threshold);
            let t71 = (simd::cbrt(t69));
            let t72 = t71 * t69;
            let t73 = ((t70).select(t65, t72));
            let t74 = t68 + t73 - f64x8::splat(2.0);
            let t75 = f64x8::splat(M_CBRT2);
            let t78 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t75 - f64x8::splat(2.0));
            let t79 = t74 * t78;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t33;
            let t86 = f64x8::splat(7.05945) * t36 + f64x8::splat(1.549425) * t33 + f64x8::splat(0.420775) * t39 + f64x8::splat(0.1562925) * t47;
            let t89 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t86;
            let t90 = (simd::ln(t89));
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t33;
            let t99 = f64x8::splat(5.1785) * t36 + f64x8::splat(0.905775) * t33 + f64x8::splat(0.1100325) * t39 + f64x8::splat(0.1241775) * t47;
            let t102 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t99;
            let t103 = (simd::ln(t102));
            let t104 = t94 * t103;
            let t106 = -f64x8::splat(0.0310907) * t81 * t90 + t55 - f64x8::splat(0.0197516734986138) * t104;
            let t107 = t79 * t106;
            let t108 = t59 * t107;
            let t110 = f64x8::splat(0.0197516734986138) * t79 * t104;
            let t111 = (simd::ln(f64x8::splat(2.0)));
            let t112 = f64x8::splat(1.0) - t111;
            let t113 = t112 * t8;
            let t114 = t64 * t64;
            let t115 = t66 * t66;
            let t116 = ((t63).select(t114, t115));
            let t117 = t71 * t71;
            let t118 = ((t70).select(t114, t117));
            let t120 = t116 / f64x8::splat(2.0) + t118 / f64x8::splat(2.0);
            let t121 = t120 * t120;
            let t122 = t121 * t120;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t126 = f64x8::splat(1.0) / t14 / t19;
            let t127 = t124 * t126;
            let t128 = f64x8::splat(1.0) / t121;
            let t129 = t75 * t128;
            let t131 = f64x8::splat(1.0) / t12;
            let t132 = t41 * t131;
            let t134 = (simd::exp(-t47 / f64x8::splat(4.0)));
            let t135 = f64x8::splat(1.0) - t134;
            let t136 = t30 * t135;
            let t137 = t132 * t136;
            let t140 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t127 * t129 * t137;
            let t142 = t128 * t41;
            let t143 = t131 * t30;
            let t144 = t142 * t143;
            let t147 = f64x8::splat(1.0) / t112;
            let t148 = t140 * t147;
            let t150 = (-t55 + t108 + t110) * t147;
            let t151 = f64x8::splat(1.0) / t122;
            let t152 = t7 * t151;
            let t154 = (simd::exp(-t150 * t152));
            let t155 = t154 - f64x8::splat(1.0);
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t7 * t156;
            let t158 = t124 * t124;
            let t159 = t157 * t158;
            let t160 = t148 * t159;
            let t162 = f64x8::splat(1.0) / t44 / t57;
            let t163 = t75 * t75;
            let t164 = t162 * t163;
            let t165 = t121 * t121;
            let t166 = f64x8::splat(1.0) / t165;
            let t168 = f64x8::splat(1.0) / t42;
            let t169 = t28 * t168;
            let t170 = t169 * t31;
            let t171 = t164 * t166 * t170;
            let t174 = t127 * t75 * t144 / f64x8::splat(96.0) + t160 * t171 / f64x8::splat(3072.0);
            let t175 = t140 * t174;
            let t176 = t147 * t7;
            let t177 = t157 * t174;
            let t179 = t148 * t177 + f64x8::splat(1.0);
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t176 * t180;
            let t183 = t175 * t181 + f64x8::splat(1.0);
            let t184 = (simd::ln(t183));
            let t187 = t113 * t122 * t184 + t108 + t110 - t55;
            let tzk0 = t27 * t187;
            acc_zk = tzk0;
            let t189 = f64x8::splat(1.0) / t14 / t13;
            let t190 = t12 * t189;
            let t193 = f64x8::splat(0.3333333333333333) * t10 * t190 * t23;
            let t194 = t17 * t20;
            let t195 = t19 * t13;
            let t196 = f64x8::splat(1.0) / t195;
            let t197 = t18 * t196;
            let t200 = ((t22).select(f64x8::splat(2.0) * t194 - f64x8::splat(2.0) * t197, f64x8::splat(0.0)));
            let t204 = t193 - f64x8::splat(1.0) * t10 * t16 * t200;
            let t205 = t13 * t204;
            let t207 = t13 * t27;
            let t208 = t31 * t189;
            let t211 = f64x8::splat(0.0011073470983333333) * t29 * t208 * t53;
            let t212 = t49 * t49;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t35 * t213;
            let t216 = f64x8::splat(1.0) / t36 * t28;
            let t217 = t12 * t31;
            let t218 = t217 * t189;
            let t219 = t216 * t218;
            let t221 = t29 * t208;
            let t223 = ((t33).sqrt());
            let t224 = t223 * t28;
            let t225 = t224 * t218;
            let t230 = t43 * t30 / t44 / t13;
            let t232 = -f64x8::splat(0.632975) * t219 - f64x8::splat(0.29896666666666666) * t221 - f64x8::splat(0.1023875) * t225 - f64x8::splat(0.08215666666666667) * t230;
            let t233 = f64x8::splat(1.0) / t52;
            let t234 = t232 * t233;
            let t236 = f64x8::splat(1.0) * t214 * t234;
            let t237 = t18 * t17;
            let t238 = t237 * t58;
            let t240 = f64x8::splat(4.0) * t238 * t107;
            let t241 = t57 * t13;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t56 * t242;
            let t245 = f64x8::splat(4.0) * t243 * t107;
            let t246 = t60 - t194;
            let t249 = ((t63).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t246));
            let t250 = -t246;
            let t253 = ((t70).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t71 * t250));
            let t255 = (t249 + t253) * t78;
            let t256 = t255 * t106;
            let t257 = t59 * t256;
            let t261 = t86 * t86;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t81 * t262;
            let t268 = -f64x8::splat(1.176575) * t219 - f64x8::splat(0.516475) * t221 - f64x8::splat(0.2103875) * t225 - f64x8::splat(0.104195) * t230;
            let t269 = f64x8::splat(1.0) / t89;
            let t270 = t268 * t269;
            let t276 = t99 * t99;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t94 * t277;
            let t283 = -f64x8::splat(0.8630833333333333) * t219 - f64x8::splat(0.301925) * t221 - f64x8::splat(0.05501625) * t225 - f64x8::splat(0.082785) * t230;
            let t284 = f64x8::splat(1.0) / t102;
            let t285 = t283 * t284;
            let t288 = f64x8::splat(0.0005323764196666666) * t29 * t208 * t90 + f64x8::splat(1.0) * t263 * t270 - t211 - t236 + f64x8::splat(0.00018311447306006544) * t29 * t208 * t103 + f64x8::splat(0.5848223622634646) * t278 * t285;
            let t289 = t79 * t288;
            let t290 = t59 * t289;
            let t292 = f64x8::splat(0.0197516734986138) * t255 * t104;
            let t293 = t79 * t28;
            let t295 = t217 * t189 * t103;
            let t297 = f64x8::splat(0.00018311447306006544) * t293 * t295;
            let t298 = t79 * t94;
            let t300 = t277 * t283 * t284;
            let t302 = f64x8::splat(0.5848223622634646) * t298 * t300;
            let t303 = t121 * t184;
            let t304 = f64x8::splat(1.0) / t66;
            let t307 = ((t63).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t304 * t246));
            let t308 = f64x8::splat(1.0) / t71;
            let t311 = ((t70).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t308 * t250));
            let t313 = t307 / f64x8::splat(2.0) + t311 / f64x8::splat(2.0);
            let t318 = f64x8::splat(1.0) / t14 / t195;
            let t319 = t124 * t318;
            let t322 = f64x8::splat(0.0019444444444444444) * t319 * t129 * t137;
            let t323 = t75 * t151;
            let t324 = t127 * t323;
            let t326 = t132 * t136 * t313;
            let t329 = t124 * t58;
            let t331 = t31 * t134;
            let t332 = t29 * t331;
            let t334 = f64x8::splat(0.0004166666666666667) * t329 * t129 * t332;
            let t335 = -t322 - f64x8::splat(0.0016666666666666668) * t324 * t326 - t334;
            let t336 = t335 * t174;
            let t340 = f64x8::splat(7.0) / f64x8::splat(288.0) * t319 * t75 * t144;
            let t341 = t30 * t313;
            let t342 = t132 * t341;
            let t345 = t335 * t147;
            let t346 = t345 * t159;
            let t349 = t148 * t7;
            let t350 = t155 * t155;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t351 * t158;
            let t353 = t352 * t162;
            let t354 = t349 * t353;
            let t355 = t163 * t166;
            let t356 = t355 * t28;
            let t357 = t168 * t31;
            let t359 = (t211 + t236 + t240 - t245 + t257 + t290 + t292 - t297 - t302) * t147;
            let t361 = t7 * t166;
            let t362 = t361 * t313;
            let t365 = f64x8::splat(3.0) * t150 * t362 - t359 * t152;
            let t366 = t365 * t154;
            let t368 = t356 * t357 * t366;
            let t372 = f64x8::splat(1.0) / t44 / t241;
            let t373 = t372 * t163;
            let t375 = t373 * t166 * t170;
            let t377 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t160 * t375;
            let t378 = t156 * t158;
            let t379 = t378 * t162;
            let t380 = t349 * t379;
            let t382 = f64x8::splat(1.0) / t165 / t120;
            let t384 = t163 * t382 * t28;
            let t386 = t384 * t357 * t313;
            let t389 = -t340 - t324 * t342 / f64x8::splat(48.0) + t346 * t171 / f64x8::splat(3072.0) - t354 * t368 / f64x8::splat(3072.0) - t377 - t380 * t386 / f64x8::splat(768.0);
            let t390 = t140 * t389;
            let t392 = t175 * t147;
            let t393 = t179 * t179;
            let t394 = f64x8::splat(1.0) / t393;
            let t395 = t7 * t394;
            let t397 = t351 * t174;
            let t398 = t397 * t366;
            let t400 = t157 * t389;
            let t402 = t148 * t400 + t345 * t177 - t349 * t398;
            let t403 = t395 * t402;
            let t405 = t336 * t181 + t390 * t181 - t392 * t403;
            let t407 = f64x8::splat(1.0) / t183;
            let t410 = t113 * t122 * t405 * t407 + f64x8::splat(3.0) * t113 * t303 * t313 + t211 + t236 + t240 - t245 + t257 + t290 + t292 - t297 - t302;
            let tvrho0 = t205 * tzk0 + t207 * t410 + tzk0;
            acc_vrho_0 = tvrho0;
            let t414 = ((t22).select(-f64x8::splat(2.0) * t194 - f64x8::splat(2.0) * t197, f64x8::splat(0.0)));
            let t418 = t193 - f64x8::splat(1.0) * t10 * t16 * t414;
            let t419 = t13 * t418;
            let t421 = -t60 - t194;
            let t424 = ((t63).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t421));
            let t425 = -t421;
            let t428 = ((t70).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t71 * t425));
            let t430 = (t424 + t428) * t78;
            let t431 = t430 * t106;
            let t432 = t59 * t431;
            let t434 = f64x8::splat(0.0197516734986138) * t430 * t104;
            let t437 = ((t63).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t304 * t421));
            let t440 = ((t70).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t308 * t425));
            let t442 = t437 / f64x8::splat(2.0) + t440 / f64x8::splat(2.0);
            let t447 = t132 * t136 * t442;
            let t450 = -t322 - f64x8::splat(0.0016666666666666668) * t324 * t447 - t334;
            let t451 = t450 * t174;
            let t453 = t30 * t442;
            let t454 = t132 * t453;
            let t457 = t450 * t147;
            let t458 = t457 * t159;
            let t462 = (t211 + t236 - t240 - t245 + t432 + t290 + t434 - t297 - t302) * t147;
            let t464 = t361 * t442;
            let t467 = f64x8::splat(3.0) * t150 * t464 - t462 * t152;
            let t468 = t467 * t154;
            let t470 = t356 * t357 * t468;
            let t474 = t384 * t357 * t442;
            let t477 = -t340 - t324 * t454 / f64x8::splat(48.0) + t458 * t171 / f64x8::splat(3072.0) - t354 * t470 / f64x8::splat(3072.0) - t377 - t380 * t474 / f64x8::splat(768.0);
            let t478 = t140 * t477;
            let t481 = t397 * t468;
            let t483 = t157 * t477;
            let t485 = t148 * t483 + t457 * t177 - t349 * t481;
            let t486 = t395 * t485;
            let t488 = t451 * t181 + t478 * t181 - t392 * t486;
            let t492 = t113 * t122 * t488 * t407 + f64x8::splat(3.0) * t113 * t303 * t442 + t211 + t236 - t240 - t245 + t290 - t297 - t302 + t432 + t434;
            let tvrho1 = t207 * t492 + t419 * tzk0 + tzk0;
            acc_vrho_1 = tvrho1;
            let t494 = t207 * t112;
            let t495 = t8 * t122;
            let t496 = t126 * t75;
            let t497 = t142 * t131;
            let t498 = t496 * t497;
            let t500 = t174 * t147 * t180;
            let t501 = t136 * t500;
            let t502 = t498 * t501;
            let t505 = t132 * t30;
            let t506 = t496 * t128 * t505;
            let t508 = t57 * t195;
            let t509 = f64x8::splat(1.0) / t508;
            let t511 = f64x8::splat(1.0) / t165 / t121;
            let t512 = t509 * t511;
            let t513 = t512 * t135;
            let t514 = t147 * t156;
            let t515 = t514 * t158;
            let t516 = t513 * t515;
            let t518 = t157 * t124;
            let t519 = t148 * t518;
            let t520 = t519 * t171;
            let t522 = t506 / f64x8::splat(96.0) + f64x8::splat(0.00020186378047070194) * t516 + t520 / f64x8::splat(1536.0);
            let t523 = t140 * t522;
            let t525 = t514 * t174;
            let t526 = t136 * t525;
            let t527 = t498 * t526;
            let t529 = t157 * t522;
            let t531 = f64x8::splat(0.008224670334241133) * t527 + t148 * t529;
            let t532 = t395 * t531;
            let t534 = f64x8::splat(0.008224670334241133) * t502 + t523 * t181 - t392 * t532;
            let t535 = t534 * t407;
            let t536 = t495 * t535;
            let tvsigma0 = t494 * t536;
            acc_vsigma_0 = tvsigma0;
            let t541 = t506 / f64x8::splat(48.0) + f64x8::splat(0.0004037275609414039) * t516 + t520 / f64x8::splat(768.0);
            let t542 = t140 * t541;
            let t545 = t157 * t541;
            let t547 = f64x8::splat(0.016449340668482266) * t527 + t148 * t545;
            let t548 = t395 * t547;
            let t550 = f64x8::splat(0.016449340668482266) * t502 + t542 * t181 - t392 * t548;
            let t551 = t550 * t407;
            let t552 = t495 * t551;
            let tvsigma1 = t494 * t552;
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
