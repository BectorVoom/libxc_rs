//! GGA_C_SOGGA11 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`
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
pub fn gga_c_sogga11_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    param_sogga11_a_0: f64,
    param_sogga11_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_sogga11_a_1 = f64x8::splat(param_sogga11_a_1);
    let param_sogga11_a_2 = f64x8::splat(param_sogga11_a_2);
    let param_sogga11_a_3 = f64x8::splat(param_sogga11_a_3);
    let param_sogga11_a_4 = f64x8::splat(param_sogga11_a_4);
    let param_sogga11_a_5 = f64x8::splat(param_sogga11_a_5);
    let param_sogga11_b_1 = f64x8::splat(param_sogga11_b_1);
    let param_sogga11_b_2 = f64x8::splat(param_sogga11_b_2);
    let param_sogga11_b_3 = f64x8::splat(param_sogga11_b_3);
    let param_sogga11_b_4 = f64x8::splat(param_sogga11_b_4);
    let param_sogga11_b_5 = f64x8::splat(param_sogga11_b_5);
    let param_sogga11_a_0 = f64x8::splat(param_sogga11_a_0);
    let param_sogga11_b_0 = f64x8::splat(param_sogga11_b_0);
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
            let t92 = -t33 + t40 * t88 + f64x8::splat(0.0197516734986138) * t60 * t85;
            let t94 = param_sogga11_a_1;
            let t95 = t45 * t45;
            let t96 = t47 * t47;
            let t97 = ((t44).select(t95, t96));
            let t98 = t52 * t52;
            let t99 = ((t51).select(t95, t98));
            let t101 = t97 / f64x8::splat(2.0) + t99 / f64x8::splat(2.0);
            let t102 = t56 * t101;
            let t104 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t106 = f64x8::splat(1.0) / t8 / t37;
            let t107 = t104 * t106;
            let t108 = t102 * t107;
            let t109 = f64x8::splat(1.0) / t3;
            let t110 = t19 * t109;
            let t111 = f64x8::splat(1.0) / t92;
            let t112 = t5 * t111;
            let t113 = t110 * t112;
            let t115 = f64x8::splat(0.0006950658458333333) * t108 * t113;
            let t116 = f64x8::splat(1.0) - t115;
            let t118 = f64x8::splat(1.0) - f64x8::splat(1.0) / t116;
            let t120 = param_sogga11_a_2;
            let t121 = t118 * t118;
            let t123 = param_sogga11_a_3;
            let t124 = t121 * t118;
            let t126 = param_sogga11_a_4;
            let t127 = t121 * t121;
            let t129 = param_sogga11_a_5;
            let t133 = param_sogga11_b_1;
            let t134 = (simd::exp(t115));
            let t135 = f64x8::splat(1.0) - t134;
            let t137 = param_sogga11_b_2;
            let t138 = t135 * t135;
            let t140 = param_sogga11_b_3;
            let t141 = t138 * t135;
            let t143 = param_sogga11_b_4;
            let t144 = t138 * t138;
            let t146 = param_sogga11_b_5;
            let t149 = t129 * t127 * t118 + t146 * t144 * t135 + t94 * t118 + t120 * t121 + t123 * t124 + t126 * t127 + t133 * t135 + t137 * t138 + t140 * t141 + t143 * t144 + param_sogga11_a_0 + param_sogga11_b_0;
            let tzk0 = t92 * t149;
            acc_zk = tzk0;
            let t151 = f64x8::splat(1.0) / t8 / t7;
            let t152 = t6 * t151;
            let t155 = f64x8::splat(0.0011073470983333333) * t4 * t152 * t31;
            let t156 = t27 * t27;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t13 * t157;
            let t160 = f64x8::splat(1.0) / t14 * t1;
            let t161 = t3 * t6;
            let t162 = t161 * t151;
            let t163 = t160 * t162;
            let t165 = t4 * t152;
            let t167 = ((t11).sqrt());
            let t168 = t167 * t1;
            let t169 = t168 * t162;
            let t174 = t21 * t5 / t22 / t7;
            let t176 = -f64x8::splat(0.632975) * t163 - f64x8::splat(0.29896666666666666) * t165 - f64x8::splat(0.1023875) * t169 - f64x8::splat(0.08215666666666667) * t174;
            let t177 = f64x8::splat(1.0) / t30;
            let t178 = t176 * t177;
            let t180 = f64x8::splat(1.0) * t158 * t178;
            let t181 = t35 * t34;
            let t182 = t181 * t39;
            let t184 = f64x8::splat(4.0) * t182 * t88;
            let t185 = t38 * t7;
            let t186 = f64x8::splat(1.0) / t185;
            let t187 = t36 * t186;
            let t189 = f64x8::splat(4.0) * t187 * t88;
            let t190 = f64x8::splat(1.0) / t37;
            let t191 = t34 * t190;
            let t192 = t41 - t191;
            let t195 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t192));
            let t196 = -t192;
            let t199 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t196));
            let t201 = (t195 + t199) * t59;
            let t202 = t201 * t87;
            let t207 = t67 * t67;
            let t208 = f64x8::splat(1.0) / t207;
            let t209 = t62 * t208;
            let t214 = -f64x8::splat(1.176575) * t163 - f64x8::splat(0.516475) * t165 - f64x8::splat(0.2103875) * t169 - f64x8::splat(0.104195) * t174;
            let t215 = f64x8::splat(1.0) / t70;
            let t216 = t214 * t215;
            let t222 = t80 * t80;
            let t223 = f64x8::splat(1.0) / t222;
            let t224 = t75 * t223;
            let t229 = -f64x8::splat(0.8630833333333333) * t163 - f64x8::splat(0.301925) * t165 - f64x8::splat(0.05501625) * t169 - f64x8::splat(0.082785) * t174;
            let t230 = f64x8::splat(1.0) / t83;
            let t231 = t229 * t230;
            let t234 = f64x8::splat(0.0005323764196666666) * t4 * t152 * t71 + f64x8::splat(1.0) * t209 * t216 - t155 - t180 + f64x8::splat(0.00018311447306006544) * t4 * t152 * t84 + f64x8::splat(0.5848223622634646) * t224 * t231;
            let t235 = t60 * t234;
            let t236 = t40 * t235;
            let t239 = t60 * t1;
            let t241 = t161 * t151 * t84;
            let t243 = f64x8::splat(0.00018311447306006544) * t239 * t241;
            let t244 = t60 * t75;
            let t246 = t223 * t229 * t230;
            let t248 = f64x8::splat(0.5848223622634646) * t244 * t246;
            let t249 = t155 + t180 + t184 - t189 + t40 * t202 + t236 + f64x8::splat(0.0197516734986138) * t201 * t85 - t243 - t248;
            let t250 = t7 * t249;
            let t252 = t7 * t92;
            let t253 = t116 * t116;
            let t254 = f64x8::splat(1.0) / t253;
            let t255 = t94 * t254;
            let t256 = f64x8::splat(1.0) / t47;
            let t259 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t256 * t192));
            let t260 = f64x8::splat(1.0) / t52;
            let t263 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t260 * t196));
            let t265 = t259 / f64x8::splat(2.0) + t263 / f64x8::splat(2.0);
            let t266 = t56 * t265;
            let t267 = t266 * t107;
            let t270 = t37 * t7;
            let t272 = f64x8::splat(1.0) / t8 / t270;
            let t273 = t104 * t272;
            let t274 = t102 * t273;
            let t276 = f64x8::splat(0.0016218203069444444) * t274 * t113;
            let t277 = t92 * t92;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t5 * t278;
            let t280 = t279 * t249;
            let t281 = t110 * t280;
            let t284 = -f64x8::splat(0.0006950658458333333) * t267 * t113 + t276 + f64x8::splat(0.0006950658458333333) * t108 * t281;
            let t286 = t120 * t118;
            let t287 = t254 * t284;
            let t290 = t123 * t121;
            let t293 = t126 * t124;
            let t296 = t129 * t127;
            let t299 = -t284;
            let t300 = t133 * t299;
            let t302 = t137 * t135;
            let t303 = t299 * t134;
            let t306 = t140 * t138;
            let t309 = t143 * t141;
            let t312 = t146 * t144;
            let t315 = -t300 * t134 + t255 * t284 + f64x8::splat(2.0) * t286 * t287 + f64x8::splat(3.0) * t290 * t287 + f64x8::splat(4.0) * t293 * t287 + f64x8::splat(5.0) * t296 * t287 - f64x8::splat(2.0) * t302 * t303 - f64x8::splat(3.0) * t306 * t303 - f64x8::splat(4.0) * t309 * t303 - f64x8::splat(5.0) * t312 * t303;
            let tvrho0 = t250 * t149 + t252 * t315 + tzk0;
            acc_vrho_0 = tvrho0;
            let t317 = -t41 - t191;
            let t320 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t317));
            let t321 = -t317;
            let t324 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t321));
            let t326 = (t320 + t324) * t59;
            let t327 = t326 * t87;
            let t331 = t155 + t180 - t184 - t189 + t40 * t327 + t236 + f64x8::splat(0.0197516734986138) * t326 * t85 - t243 - t248;
            let t332 = t7 * t331;
            let t336 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t256 * t317));
            let t339 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t260 * t321));
            let t341 = t336 / f64x8::splat(2.0) + t339 / f64x8::splat(2.0);
            let t342 = t56 * t341;
            let t343 = t342 * t107;
            let t346 = t279 * t331;
            let t347 = t110 * t346;
            let t350 = -f64x8::splat(0.0006950658458333333) * t343 * t113 + t276 + f64x8::splat(0.0006950658458333333) * t108 * t347;
            let t352 = t254 * t350;
            let t361 = -t350;
            let t362 = t133 * t361;
            let t364 = t361 * t134;
            let t373 = -t362 * t134 + t255 * t350 + f64x8::splat(2.0) * t286 * t352 + f64x8::splat(3.0) * t290 * t352 + f64x8::splat(4.0) * t293 * t352 + f64x8::splat(5.0) * t296 * t352 - f64x8::splat(2.0) * t302 * t364 - f64x8::splat(3.0) * t306 * t364 - f64x8::splat(4.0) * t309 * t364 - f64x8::splat(5.0) * t312 * t364;
            let tvrho1 = t332 * t149 + t252 * t373 + tzk0;
            acc_vrho_1 = tvrho1;
            let t375 = t255 * t102;
            let t376 = t106 * t19;
            let t377 = t109 * t5;
            let t378 = t377 * t111;
            let t379 = t376 * t378;
            let t380 = t375 * t379;
            let t382 = t254 * t56;
            let t383 = t382 * t101;
            let t384 = t286 * t383;
            let t385 = t384 * t379;
            let t387 = t290 * t383;
            let t388 = t387 * t379;
            let t390 = t293 * t383;
            let t391 = t390 * t379;
            let t393 = t296 * t383;
            let t394 = t393 * t379;
            let t396 = t133 * t56;
            let t397 = t101 * t106;
            let t400 = t110 * t112 * t134;
            let t401 = t396 * t397 * t400;
            let t403 = t102 * t106;
            let t404 = t302 * t403;
            let t405 = t404 * t400;
            let t407 = t306 * t403;
            let t408 = t407 * t400;
            let t410 = t309 * t403;
            let t411 = t410 * t400;
            let t413 = t312 * t403;
            let t414 = t413 * t400;
            let t416 = -f64x8::splat(0.0006950658458333333) * t380 - f64x8::splat(0.0013901316916666666) * t385 - f64x8::splat(0.0020851975375) * t388 - f64x8::splat(0.0027802633833333332) * t391 - f64x8::splat(0.0034753292291666666) * t394 - f64x8::splat(0.0006950658458333333) * t401 - f64x8::splat(0.0013901316916666666) * t405 - f64x8::splat(0.0020851975375) * t408 - f64x8::splat(0.0027802633833333332) * t411 - f64x8::splat(0.0034753292291666666) * t414;
            let tvsigma0 = t252 * t416;
            acc_vsigma_0 = tvsigma0;
            let t427 = -f64x8::splat(0.0013901316916666666) * t380 - f64x8::splat(0.0027802633833333332) * t385 - f64x8::splat(0.004170395075) * t388 - f64x8::splat(0.0055605267666666664) * t391 - f64x8::splat(0.006950658458333333) * t394 - f64x8::splat(0.0013901316916666666) * t401 - f64x8::splat(0.0027802633833333332) * t405 - f64x8::splat(0.004170395075) * t408 - f64x8::splat(0.0055605267666666664) * t411 - f64x8::splat(0.006950658458333333) * t414;
            let tvsigma1 = t252 * t427;
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
