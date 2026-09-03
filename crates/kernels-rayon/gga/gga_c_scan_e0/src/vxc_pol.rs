//! GGA_C_SCAN_E0 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`
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
pub fn gga_c_scan_e0_vxc_pol(
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
            let t92 = (simd::ln(f64x8::splat(2.0)));
            let t93 = f64x8::splat(1.0) - t92;
            let t94 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t96 = t93 / t94;
            let t97 = t45 * t45;
            let t98 = t47 * t47;
            let t99 = ((t44).select(t97, t98));
            let t100 = t52 * t52;
            let t101 = ((t51).select(t97, t100));
            let t103 = t99 / f64x8::splat(2.0) + t101 / f64x8::splat(2.0);
            let t104 = t103 * t103;
            let t105 = t104 * t103;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t107 * t110;
            let t112 = f64x8::splat(1.0) / t93;
            let t114 = (-t33 + t89 + t91) * t112;
            let t115 = f64x8::splat(1.0) / t105;
            let t116 = t94 * t115;
            let t118 = (simd::exp(-t114 * t116));
            let t119 = t118 - f64x8::splat(1.0);
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t112 * t120;
            let t123 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t124 = t121 * t123;
            let t125 = t111 * t124;
            let t127 = f64x8::splat(1.0) / t8 / t37;
            let t128 = t127 * t56;
            let t129 = f64x8::splat(1.0) / t104;
            let t131 = f64x8::splat(1.0) / t3;
            let t132 = t19 * t131;
            let t133 = t132 * t5;
            let t137 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t125 * t128 * t129 * t133;
            let t138 = ((t137).sqrt().sqrt());
            let t140 = f64x8::splat(1.0) - f64x8::splat(1.0) / t138;
            let t143 = f64x8::splat(1.0) + f64x8::splat(1.0) * t140 * t119;
            let t144 = (simd::ln(t143));
            let t146 = t96 * t105 * t144;
            let tzk0 = -t33 + t89 + t91 + t146;
            acc_zk = tzk0;
            let t148 = f64x8::splat(1.0) / t8 / t7;
            let t149 = t6 * t148;
            let t151 = t4 * t149 * t31;
            let t152 = f64x8::splat(0.0011073470983333333) * t151;
            let t153 = t27 * t27;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t13 * t154;
            let t157 = f64x8::splat(1.0) / t14 * t1;
            let t158 = t3 * t6;
            let t159 = t158 * t148;
            let t160 = t157 * t159;
            let t162 = t4 * t149;
            let t164 = ((t11).sqrt());
            let t165 = t164 * t1;
            let t166 = t165 * t159;
            let t171 = t21 * t5 / t22 / t7;
            let t173 = -f64x8::splat(0.632975) * t160 - f64x8::splat(0.29896666666666666) * t162 - f64x8::splat(0.1023875) * t166 - f64x8::splat(0.08215666666666667) * t171;
            let t174 = f64x8::splat(1.0) / t30;
            let t175 = t173 * t174;
            let t176 = t155 * t175;
            let t177 = f64x8::splat(1.0) * t176;
            let t178 = t35 * t34;
            let t179 = t178 * t39;
            let t180 = t179 * t88;
            let t181 = f64x8::splat(4.0) * t180;
            let t182 = t38 * t7;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t36 * t183;
            let t185 = t184 * t88;
            let t186 = f64x8::splat(4.0) * t185;
            let t187 = f64x8::splat(1.0) / t37;
            let t188 = t34 * t187;
            let t189 = t41 - t188;
            let t192 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t189));
            let t193 = -t189;
            let t196 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t193));
            let t198 = (t192 + t196) * t59;
            let t199 = t198 * t87;
            let t200 = t40 * t199;
            let t204 = t67 * t67;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t62 * t205;
            let t211 = -f64x8::splat(1.176575) * t160 - f64x8::splat(0.516475) * t162 - f64x8::splat(0.2103875) * t166 - f64x8::splat(0.104195) * t171;
            let t212 = f64x8::splat(1.0) / t70;
            let t213 = t211 * t212;
            let t219 = t80 * t80;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t75 * t220;
            let t226 = -f64x8::splat(0.8630833333333333) * t160 - f64x8::splat(0.301925) * t162 - f64x8::splat(0.05501625) * t166 - f64x8::splat(0.082785) * t171;
            let t227 = f64x8::splat(1.0) / t83;
            let t228 = t226 * t227;
            let t231 = f64x8::splat(0.0005323764196666666) * t4 * t149 * t71 + f64x8::splat(1.0) * t206 * t213 - t152 - t177 + f64x8::splat(0.00018311447306006544) * t4 * t149 * t84 + f64x8::splat(0.5848223622634646) * t221 * t228;
            let t232 = t60 * t231;
            let t233 = t40 * t232;
            let t234 = t198 * t85;
            let t235 = f64x8::splat(0.0197516734986138) * t234;
            let t236 = t60 * t1;
            let t238 = t158 * t148 * t84;
            let t239 = t236 * t238;
            let t240 = f64x8::splat(0.00018311447306006544) * t239;
            let t241 = t60 * t75;
            let t243 = t220 * t226 * t227;
            let t244 = t241 * t243;
            let t245 = f64x8::splat(0.5848223622634646) * t244;
            let t246 = t104 * t144;
            let t247 = f64x8::splat(1.0) / t47;
            let t250 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t247 * t189));
            let t251 = f64x8::splat(1.0) / t52;
            let t254 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t251 * t193));
            let t256 = t250 / f64x8::splat(2.0) + t254 / f64x8::splat(2.0);
            let t258 = t96 * t246 * t256;
            let t259 = f64x8::splat(3.0) * t258;
            let t261 = f64x8::splat(1.0) / t138 / t137;
            let t262 = t37 * t7;
            let t264 = f64x8::splat(1.0) / t22 / t262;
            let t265 = t264 * t110;
            let t267 = t120 * t123;
            let t268 = t56 * t129;
            let t269 = t267 * t268;
            let t271 = f64x8::splat(0.002743937159556463) * t265 * t112 * t269;
            let t272 = t109 * t109;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t107 * t273;
            let t275 = t274 * t121;
            let t276 = t123 * t264;
            let t279 = f64x8::splat(0.004878720269691391) * t275 * t276 * t268;
            let t280 = t111 * t112;
            let t281 = t119 * t119;
            let t282 = f64x8::splat(1.0) / t281;
            let t283 = t282 * t123;
            let t285 = t280 * t283 * t127;
            let t286 = t268 * t19;
            let t287 = t131 * t5;
            let t289 = (t152 + t177 + t181 - t186 + t200 + t233 + t235 - t240 - t245) * t112;
            let t291 = t104 * t104;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t94 * t292;
            let t294 = t293 * t256;
            let t297 = f64x8::splat(3.0) * t114 * t294 - t116 * t289;
            let t298 = t297 * t118;
            let t300 = t286 * t287 * t298;
            let t304 = f64x8::splat(1.0) / t8 / t262;
            let t305 = t304 * t56;
            let t309 = f64x8::splat(0.0640252003896508) * t125 * t305 * t129 * t133;
            let t311 = t280 * t267 * t127;
            let t312 = t56 * t115;
            let t313 = t312 * t19;
            let t315 = t313 * t287 * t256;
            let t318 = -t271 + t279 - f64x8::splat(0.027439371595564633) * t285 * t300 - t309 - f64x8::splat(0.054878743191129266) * t311 * t315;
            let t319 = t261 * t318;
            let t325 = f64x8::splat(0.25) * t319 * t119 + f64x8::splat(1.0) * t140 * t297 * t118;
            let t327 = f64x8::splat(1.0) / t143;
            let t329 = t96 * t105 * t325 * t327;
            let t330 = t152 + t177 + t181 - t186 + t200 + t233 + t235 - t240 - t245 + t259 + t329;
            let tvrho0 = t330 * t7 + t146 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t332 = -t41 - t188;
            let t335 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t332));
            let t336 = -t332;
            let t339 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t336));
            let t341 = (t335 + t339) * t59;
            let t342 = t341 * t87;
            let t343 = t40 * t342;
            let t344 = t341 * t85;
            let t345 = f64x8::splat(0.0197516734986138) * t344;
            let t348 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t247 * t332));
            let t351 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t251 * t336));
            let t353 = t348 / f64x8::splat(2.0) + t351 / f64x8::splat(2.0);
            let t355 = t96 * t246 * t353;
            let t356 = f64x8::splat(3.0) * t355;
            let t358 = (t152 + t177 - t181 - t186 + t343 + t233 + t345 - t240 - t245) * t112;
            let t360 = t293 * t353;
            let t363 = f64x8::splat(3.0) * t114 * t360 - t116 * t358;
            let t364 = t363 * t118;
            let t366 = t286 * t287 * t364;
            let t370 = t313 * t287 * t353;
            let t373 = -t271 + t279 - f64x8::splat(0.027439371595564633) * t285 * t366 - t309 - f64x8::splat(0.054878743191129266) * t311 * t370;
            let t374 = t261 * t373;
            let t377 = t140 * t363;
            let t380 = f64x8::splat(0.25) * t374 * t119 + f64x8::splat(1.0) * t377 * t118;
            let t383 = t96 * t105 * t380 * t327;
            let t384 = t152 + t177 - t181 - t186 + t343 + t233 + t345 - t240 - t245 + t356 + t383;
            let tvrho1 = t384 * t7 + t146 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t386 = t148 * t103;
            let t387 = t261 * t107;
            let t388 = t387 * t110;
            let t389 = t386 * t388;
            let t390 = t56 * t19;
            let t391 = t287 * t327;
            let t392 = t390 * t391;
            let t393 = t389 * t392;
            let tvsigma0 = f64x8::splat(0.0006950474021161377) * t393;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0013900948042322753) * t393;
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
