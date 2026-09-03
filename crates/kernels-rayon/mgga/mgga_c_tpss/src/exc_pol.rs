//! MGGA_C_TPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_tpss.c`
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
pub fn mgga_c_tpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_beta: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_C0_c_0 = f64x8::splat(param_C0_c_0);
    let param_C0_c_1 = f64x8::splat(param_C0_c_1);
    let param_C0_c_2 = f64x8::splat(param_C0_c_2);
    let param_C0_c_3 = f64x8::splat(param_C0_c_3);
    let param_beta = f64x8::splat(param_beta);
    let param_d = f64x8::splat(param_d);
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
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t7 = (((f64x8::splat(0.0)).simd_lt(t5)).select(t5, -t5));
            let t8 = (-t7).simd_le(-f64x8::splat(0.999999999999));
            let t9 = param_C0_c_0;
            let t10 = param_C0_c_1;
            let t11 = param_C0_c_2;
            let t12 = param_C0_c_3;
            let t14 = t2 * t2;
            let t15 = t10 * t14;
            let t16 = t3 * t3;
            let t17 = f64x8::splat(1.0) / t16;
            let t19 = t14 * t14;
            let t20 = t11 * t19;
            let t21 = t16 * t16;
            let t22 = f64x8::splat(1.0) / t21;
            let t25 = t12 * t19 * t14;
            let t26 = t21 * t16;
            let t27 = f64x8::splat(1.0) / t26;
            let t29 = t15 * t17 + t20 * t22 + t25 * t27 + t9;
            let t30 = f64x8::splat(1.0) + t5;
            let t31 = (t30).simd_le(zeta_threshold);
            let t32 = zeta_threshold - f64x8::splat(1.0);
            let t33 = f64x8::splat(1.0) - t5;
            let t34 = (t33).simd_le(zeta_threshold);
            let t36 = ((t31).select(t32, (t34).select(-t32, t5)));
            let t37 = t36 * t36;
            let t38 = f64x8::splat(1.0) - t37;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t44 = v_sigma0 * t43;
            let t45 = f64x8::splat(1.0) + t36;
            let t46 = t45 / f64x8::splat(2.0);
            let t47 = (simd::cbrt(t46));
            let t48 = t47 * t47;
            let t49 = t48 * t46;
            let t51 = v_rho1 * v_rho1;
            let t52 = (simd::cbrt(v_rho1));
            let t53 = t52 * t52;
            let t55 = f64x8::splat(1.0) / t53 / t51;
            let t56 = v_sigma2 * t55;
            let t57 = f64x8::splat(1.0) - t36;
            let t58 = t57 / f64x8::splat(2.0);
            let t59 = (simd::cbrt(t58));
            let t60 = t59 * t59;
            let t61 = t60 * t58;
            let t64 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t65 = (simd::cbrt(t3));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t16;
            let t69 = t64 * t68;
            let t70 = t44 * t49 + t56 * t61 - t69;
            let t71 = t38 * t70;
            let t72 = f64x8::splat(M_CBRT3);
            let t73 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t74 = (simd::cbrt(t73));
            let t75 = t74 * t74;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t72 * t76;
            let t78 = (simd::cbrt(t45));
            let t79 = t78 * t45;
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = (simd::cbrt(t57));
            let t82 = t81 * t57;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t80 + t83;
            let t85 = t77 * t84;
            let t88 = f64x8::splat(1.0) + t71 * t85 / f64x8::splat(24.0);
            let t89 = t88 * t88;
            let t90 = t89 * t89;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = ((t8).select(t9 + t10 + t11 + t12, t29 * t91));
            let t94 = f64x8::splat(1.0) + t93;
            let t96 = f64x8::splat(1.0) / t41 / v_rho0;
            let t97 = v_tau0 * t96;
            let t98 = t30 / f64x8::splat(2.0);
            let t99 = (simd::cbrt(t98));
            let t100 = t99 * t99;
            let t101 = t100 * t98;
            let t104 = f64x8::splat(1.0) / t53 / v_rho1;
            let t105 = v_tau1 * t104;
            let t106 = t33 / f64x8::splat(2.0);
            let t107 = (simd::cbrt(t106));
            let t108 = t107 * t107;
            let t109 = t108 * t106;
            let t111 = t101 * t97 + t105 * t109;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = t69 * t112 / f64x8::splat(8.0);
            let t115 = (f64x8::splat(1.0)).simd_lt(t114);
            let t116 = ((t115).select(f64x8::splat(1.0), t114));
            let t117 = t116 * t116;
            let t118 = t94 * t117;
            let t120 = ((v_rho0).simd_le(dens_threshold)) | (t31);
            let t121 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t122 = (simd::cbrt(t121));
            let t123 = t72 * t122;
            let t124 = f64x8::splat(M_CBRT4);
            let t125 = t124 * t124;
            let t126 = f64x8::splat(1.0) / t65;
            let t127 = t125 * t126;
            let t128 = t123 * t127;
            let t130 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t128;
            let t131 = ((t128).sqrt());
            let t134 = ((t128) * (t128).sqrt());
            let t136 = t72 * t72;
            let t137 = t122 * t122;
            let t138 = t136 * t137;
            let t139 = f64x8::splat(1.0) / t66;
            let t140 = t124 * t139;
            let t141 = t138 * t140;
            let t143 = f64x8::splat(3.79785) * t131 + f64x8::splat(0.8969) * t128 + f64x8::splat(0.204775) * t134 + f64x8::splat(0.123235) * t141;
            let t146 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t143;
            let t147 = (simd::ln(t146));
            let t149 = f64x8::splat(0.0621814) * t130 * t147;
            let t150 = t37 * t37;
            let t151 = (t45).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(zeta_threshold));
            let t153 = t152 * zeta_threshold;
            let t154 = ((t151).select(t153, t79));
            let t155 = (t57).simd_le(zeta_threshold);
            let t156 = ((t155).select(t153, t82));
            let t157 = t154 + t156 - f64x8::splat(2.0);
            let t158 = t150 * t157;
            let t159 = f64x8::splat(M_CBRT2);
            let t162 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t159 - f64x8::splat(2.0));
            let t164 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t128;
            let t169 = f64x8::splat(7.05945) * t131 + f64x8::splat(1.549425) * t128 + f64x8::splat(0.420775) * t134 + f64x8::splat(0.1562925) * t141;
            let t172 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t169;
            let t173 = (simd::ln(t172));
            let t177 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t128;
            let t182 = f64x8::splat(5.1785) * t131 + f64x8::splat(0.905775) * t128 + f64x8::splat(0.1100325) * t134 + f64x8::splat(0.1241775) * t141;
            let t185 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t182;
            let t186 = (simd::ln(t185));
            let t187 = t177 * t186;
            let t189 = -f64x8::splat(0.0310907) * t164 * t173 + t149 - f64x8::splat(0.0197516734986138) * t187;
            let t190 = t162 * t189;
            let t191 = t158 * t190;
            let t192 = t157 * t162;
            let t194 = f64x8::splat(0.0197516734986138) * t192 * t187;
            let t195 = (simd::ln(f64x8::splat(2.0)));
            let t196 = f64x8::splat(1.0) - t195;
            let t197 = f64x8::splat(1.0) / t73;
            let t198 = t196 * t197;
            let t199 = t152 * t152;
            let t200 = t78 * t78;
            let t201 = ((t151).select(t199, t200));
            let t202 = t81 * t81;
            let t203 = ((t155).select(t199, t202));
            let t205 = t201 / f64x8::splat(2.0) + t203 / f64x8::splat(2.0);
            let t206 = t205 * t205;
            let t207 = t206 * t205;
            let t209 = f64x8::splat(1.0) / t65 / t16;
            let t210 = t64 * t209;
            let t211 = t210 * t159;
            let t212 = f64x8::splat(1.0) / t206;
            let t214 = f64x8::splat(1.0) / t122;
            let t215 = t214 * t124;
            let t216 = t212 * t136 * t215;
            let t219 = f64x8::splat(1.0) / t196;
            let t220 = param_beta * t219;
            let t222 = (-t149 + t191 + t194) * t219;
            let t223 = f64x8::splat(1.0) / t207;
            let t224 = t73 * t223;
            let t226 = (simd::exp(-t222 * t224));
            let t227 = t226 - f64x8::splat(1.0);
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t73 * t228;
            let t230 = t64 * t64;
            let t232 = t220 * t229 * t230;
            let t234 = f64x8::splat(1.0) / t66 / t21;
            let t235 = t159 * t159;
            let t236 = t234 * t235;
            let t237 = t206 * t206;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t236 * t238;
            let t240 = f64x8::splat(1.0) / t137;
            let t241 = t72 * t240;
            let t242 = t241 * t125;
            let t243 = t239 * t242;
            let t246 = t211 * t216 / f64x8::splat(96.0) + t232 * t243 / f64x8::splat(3072.0);
            let t247 = param_beta * t246;
            let t248 = t219 * t73;
            let t251 = t220 * t229 * t246 + f64x8::splat(1.0);
            let t252 = f64x8::splat(1.0) / t251;
            let t253 = t248 * t252;
            let t255 = t247 * t253 + f64x8::splat(1.0);
            let t256 = (simd::ln(t255));
            let t259 = t198 * t207 * t256 - t149 + t191 + t194;
            let t262 = t123 * t125;
            let t263 = t126 * t159;
            let t264 = f64x8::splat(1.0) / t45;
            let t265 = (simd::cbrt(t264));
            let t267 = t262 * t263 * t265;
            let t269 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t267;
            let t270 = ((t267).sqrt());
            let t273 = ((t267) * (t267).sqrt());
            let t275 = t138 * t124;
            let t276 = t139 * t235;
            let t277 = t265 * t265;
            let t279 = t275 * t276 * t277;
            let t281 = f64x8::splat(3.79785) * t270 + f64x8::splat(0.8969) * t267 + f64x8::splat(0.204775) * t273 + f64x8::splat(0.123235) * t279;
            let t284 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t281;
            let t285 = (simd::ln(t284));
            let t287 = f64x8::splat(0.0621814) * t269 * t285;
            let t288 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t290 = ((t288).select(t153, f64x8::splat(2.0) * t159));
            let t291 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t292 = ((t291).select(t153, f64x8::splat(0.0)));
            let t294 = (t290 + t292 - f64x8::splat(2.0)) * t162;
            let t296 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t267;
            let t301 = f64x8::splat(7.05945) * t270 + f64x8::splat(1.549425) * t267 + f64x8::splat(0.420775) * t273 + f64x8::splat(0.1562925) * t279;
            let t304 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t301;
            let t305 = (simd::ln(t304));
            let t309 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t267;
            let t314 = f64x8::splat(5.1785) * t270 + f64x8::splat(0.905775) * t267 + f64x8::splat(0.1100325) * t273 + f64x8::splat(0.1241775) * t279;
            let t317 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t314;
            let t318 = (simd::ln(t317));
            let t319 = t309 * t318;
            let t322 = t294 * (-f64x8::splat(0.0310907) * t296 * t305 + t287 - f64x8::splat(0.0197516734986138) * t319);
            let t324 = f64x8::splat(0.0197516734986138) * t294 * t319;
            let t325 = ((t288).select(t199, t235));
            let t326 = ((t291).select(t199, f64x8::splat(0.0)));
            let t328 = t325 / f64x8::splat(2.0) + t326 / f64x8::splat(2.0);
            let t329 = t328 * t328;
            let t330 = t329 * t328;
            let t331 = f64x8::splat(1.0) / t329;
            let t332 = t331 * t136;
            let t333 = t44 * t332;
            let t334 = f64x8::splat(1.0) / t265;
            let t336 = t215 * t65 * t334;
            let t339 = t220 * t73;
            let t342 = f64x8::splat(1.0) / t330;
            let t343 = t73 * t342;
            let t345 = (simd::exp(-(-t287 + t322 + t324) * t219 * t343));
            let t346 = t345 - f64x8::splat(1.0);
            let t347 = f64x8::splat(1.0) / t346;
            let t348 = v_sigma0 * v_sigma0;
            let t349 = t347 * t348;
            let t350 = t39 * t39;
            let t351 = t350 * v_rho0;
            let t353 = f64x8::splat(1.0) / t40 / t351;
            let t355 = t339 * t349 * t353;
            let t356 = t329 * t329;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t357 * t72;
            let t359 = t358 * t240;
            let t360 = t125 * t66;
            let t361 = f64x8::splat(1.0) / t277;
            let t363 = t359 * t360 * t361;
            let t366 = t333 * t336 / f64x8::splat(96.0) + t355 * t363 / f64x8::splat(3072.0);
            let t367 = param_beta * t366;
            let t368 = t73 * t347;
            let t371 = t220 * t366 * t368 + f64x8::splat(1.0);
            let t372 = f64x8::splat(1.0) / t371;
            let t373 = t248 * t372;
            let t375 = t367 * t373 + f64x8::splat(1.0);
            let t376 = (simd::ln(t375));
            let t379 = t198 * t330 * t376 - t287 + t322 + t324;
            let t380 = (t259).simd_lt(t379);
            let t381 = ((t380).select(t379, t259));
            let t384 = ((t120).select(t259 * t30 / f64x8::splat(2.0), t381 * t45 / f64x8::splat(2.0)));
            let t386 = ((v_rho1).simd_le(dens_threshold)) | (t34);
            let t389 = f64x8::splat(1.0) / t57;
            let t390 = (simd::cbrt(t389));
            let t392 = t262 * t263 * t390;
            let t394 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t392;
            let t395 = ((t392).sqrt());
            let t398 = ((t392) * (t392).sqrt());
            let t400 = t390 * t390;
            let t402 = t275 * t276 * t400;
            let t404 = f64x8::splat(3.79785) * t395 + f64x8::splat(0.8969) * t392 + f64x8::splat(0.204775) * t398 + f64x8::splat(0.123235) * t402;
            let t407 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t404;
            let t408 = (simd::ln(t407));
            let t410 = f64x8::splat(0.0621814) * t394 * t408;
            let t412 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t392;
            let t417 = f64x8::splat(7.05945) * t395 + f64x8::splat(1.549425) * t392 + f64x8::splat(0.420775) * t398 + f64x8::splat(0.1562925) * t402;
            let t420 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t417;
            let t421 = (simd::ln(t420));
            let t425 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t392;
            let t430 = f64x8::splat(5.1785) * t395 + f64x8::splat(0.905775) * t392 + f64x8::splat(0.1100325) * t398 + f64x8::splat(0.1241775) * t402;
            let t433 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t430;
            let t434 = (simd::ln(t433));
            let t435 = t425 * t434;
            let t438 = t294 * (-f64x8::splat(0.0310907) * t412 * t421 + t410 - f64x8::splat(0.0197516734986138) * t435);
            let t440 = f64x8::splat(0.0197516734986138) * t294 * t435;
            let t441 = t56 * t332;
            let t442 = f64x8::splat(1.0) / t390;
            let t444 = t215 * t65 * t442;
            let t450 = (simd::exp(-(-t410 + t438 + t440) * t219 * t343));
            let t451 = t450 - f64x8::splat(1.0);
            let t452 = f64x8::splat(1.0) / t451;
            let t453 = v_sigma2 * v_sigma2;
            let t454 = t452 * t453;
            let t455 = t51 * t51;
            let t456 = t455 * v_rho1;
            let t458 = f64x8::splat(1.0) / t52 / t456;
            let t460 = t339 * t454 * t458;
            let t461 = f64x8::splat(1.0) / t400;
            let t463 = t359 * t360 * t461;
            let t466 = t441 * t444 / f64x8::splat(96.0) + t460 * t463 / f64x8::splat(3072.0);
            let t467 = param_beta * t466;
            let t468 = t73 * t452;
            let t471 = t220 * t466 * t468 + f64x8::splat(1.0);
            let t472 = f64x8::splat(1.0) / t471;
            let t473 = t248 * t472;
            let t475 = t467 * t473 + f64x8::splat(1.0);
            let t476 = (simd::ln(t475));
            let t479 = t198 * t330 * t476 - t410 + t438 + t440;
            let t480 = (t259).simd_lt(t479);
            let t481 = ((t480).select(t479, t259));
            let t484 = ((t386).select(t259 * t33 / f64x8::splat(2.0), t481 * t57 / f64x8::splat(2.0)));
            let t485 = t384 + t484;
            let t488 = t117 * t93 + f64x8::splat(1.0);
            let t489 = t19 * t22;
            let t490 = (simd::cbrt(t30));
            let t491 = t490 * t30;
            let t492 = ((t31).select(t153, t491));
            let t493 = (simd::cbrt(t33));
            let t494 = t493 * t33;
            let t495 = ((t34).select(t153, t494));
            let t496 = t492 + t495 - f64x8::splat(2.0);
            let t497 = t496 * t162;
            let t498 = t497 * t189;
            let t499 = t489 * t498;
            let t501 = f64x8::splat(0.0197516734986138) * t497 * t187;
            let t502 = t490 * t490;
            let t503 = ((t31).select(t199, t502));
            let t504 = t493 * t493;
            let t505 = ((t34).select(t199, t504));
            let t507 = t503 / f64x8::splat(2.0) + t505 / f64x8::splat(2.0);
            let t508 = t507 * t507;
            let t509 = t508 * t507;
            let t510 = f64x8::splat(1.0) / t508;
            let t512 = t510 * t136 * t215;
            let t516 = (-t149 + t499 + t501) * t219;
            let t517 = f64x8::splat(1.0) / t509;
            let t518 = t73 * t517;
            let t520 = (simd::exp(-t516 * t518));
            let t521 = t520 - f64x8::splat(1.0);
            let t522 = f64x8::splat(1.0) / t521;
            let t523 = t73 * t522;
            let t525 = t220 * t523 * t230;
            let t526 = t508 * t508;
            let t527 = f64x8::splat(1.0) / t526;
            let t528 = t236 * t527;
            let t529 = t528 * t242;
            let t532 = t211 * t512 / f64x8::splat(96.0) + t525 * t529 / f64x8::splat(3072.0);
            let t533 = param_beta * t532;
            let t536 = t220 * t523 * t532 + f64x8::splat(1.0);
            let t537 = f64x8::splat(1.0) / t536;
            let t538 = t248 * t537;
            let t540 = t533 * t538 + f64x8::splat(1.0);
            let t541 = (simd::ln(t540));
            let t544 = t198 * t509 * t541 - t149 + t499 + t501;
            let t546 = -t118 * t485 + t488 * t544;
            let t547 = param_d * t546;
            let t548 = t117 * t116;
            let t550 = t547 * t548 + f64x8::splat(1.0);
            let tzk0 = t546 * t550;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
