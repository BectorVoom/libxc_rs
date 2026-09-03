//! MGGA_C_REVSCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revscan.c`
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
pub fn mgga_c_revscan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t128 = f64x8::splat(1.0) / t9 / t38;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t135 = t129 * t130 * t134;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.054878743191129266) * t126 * t135;
            let t139 = ((t138).sqrt().sqrt());
            let t142 = t108 * t108;
            let t143 = t110 * t110;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t142 * t144;
            let t146 = t94 * t94;
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t120 * t120;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t147 * t149;
            let t151 = t124 * t124;
            let t152 = t150 * t151;
            let t153 = t145 * t152;
            let t155 = f64x8::splat(1.0) / t23 / t39;
            let t156 = t57 * t57;
            let t157 = t155 * t156;
            let t158 = t105 * t105;
            let t159 = f64x8::splat(1.0) / t158;
            let t160 = t157 * t159;
            let t161 = f64x8::splat(1.0) / t21;
            let t162 = t2 * t161;
            let t163 = t162 * t7;
            let t164 = t160 * t163;
            let t167 = f64x8::splat(1.0) + f64x8::splat(0.011293786703392187) * t153 * t164;
            let t168 = (simd::pow(t167, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t171 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139 / f64x8::splat(2.0) - f64x8::splat(1.0) / t168 / f64x8::splat(2.0);
            let t174 = f64x8::splat(1.0) + f64x8::splat(1.0) * t171 * t120;
            let t175 = (simd::ln(t174));
            let t177 = t97 * t106 * t175;
            let t178 = (simd::cbrt(v_rho0));
            let t179 = t178 * t178;
            let t181 = f64x8::splat(1.0) / t179 / v_rho0;
            let t182 = v_tau0 * t181;
            let t183 = t44 / f64x8::splat(2.0);
            let t184 = (simd::cbrt(t183));
            let t185 = t184 * t184;
            let t186 = t185 * t183;
            let t188 = (simd::cbrt(v_rho1));
            let t189 = t188 * t188;
            let t191 = f64x8::splat(1.0) / t189 / v_rho1;
            let t192 = v_tau1 * t191;
            let t193 = t51 / f64x8::splat(2.0);
            let t194 = (simd::cbrt(t193));
            let t195 = t194 * t194;
            let t196 = t195 * t193;
            let t199 = f64x8::splat(1.0) / t23 / t38;
            let t203 = f64x8::splat(M_CBRT6);
            let t204 = (t182 * t186 + t192 * t196 - t124 * t199 / f64x8::splat(8.0)) * t203;
            let t205 = (simd::cbrt(t95));
            let t206 = t205 * t205;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t186 + t196;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t207 * t209;
            let t212 = f64x8::splat(5.0) / f64x8::splat(9.0) * t204 * t210;
            let t213 = (t212).simd_le(f64x8::splat(1.0));
            let t214 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t217 = t214 / (-t214 + f64x8::splat(1.131));
            let t218 = (-t217).simd_lt(t212);
            let t219 = (t212).simd_lt(-t217);
            let t220 = ((t219).select(t212, -t217));
            let t221 = f64x8::splat(1.0) - t220;
            let t222 = f64x8::splat(1.0) / t221;
            let t225 = (simd::exp(-f64x8::splat(1.131) * t220 * t222));
            let t226 = ((t218).select(f64x8::splat(0.0), t225));
            let t228 = (simd::ln(f64x8::splat(0.7299270072992701) * f64x8::splat(f64::EPSILON)));
            let t231 = (-t228 + f64x8::splat(1.7)) / t228;
            let t232 = (t212).simd_lt(-t231);
            let t233 = ((t232).select(-t231, t212));
            let t234 = f64x8::splat(1.0) - t233;
            let t237 = (simd::exp(f64x8::splat(1.7) / t234));
            let t239 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(1.37) * t237));
            let t240 = ((t213).select(t226, t239));
            let t243 = f64x8::splat(1.0) + f64x8::splat(0.033115) * t15 + f64x8::splat(0.04168) * t12;
            let t244 = f64x8::splat(1.0) / t243;
            let t247 = (simd::exp(f64x8::splat(1.0) * t244));
            let t248 = t247 - f64x8::splat(1.0);
            let t249 = t203 * t207;
            let t250 = t156 * t124;
            let t254 = f64x8::splat(1.0) + f64x8::splat(0.04267528420875272) * t249 * t250 * t199;
            let t255 = ((t254).sqrt().sqrt());
            let t258 = t203 * t203;
            let t260 = f64x8::splat(1.0) / t205 / t95;
            let t261 = t258 * t260;
            let t262 = t57 * t151;
            let t263 = t39 * t8;
            let t265 = f64x8::splat(1.0) / t9 / t263;
            let t269 = f64x8::splat(1.0) + f64x8::splat(0.004552949705744548) * t261 * t262 * t265;
            let t270 = (simd::pow(t269, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t273 = f64x8::splat(1.0) - f64x8::splat(1.0) / t255 / f64x8::splat(2.0) - f64x8::splat(1.0) / t270 / f64x8::splat(2.0);
            let t275 = t248 * t273 + f64x8::splat(1.0);
            let t276 = (simd::ln(t275));
            let t278 = -f64x8::splat(0.030197) * t244 + f64x8::splat(0.030197) * t276;
            let t282 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t283 = t278 * t282;
            let t284 = t37 * t37;
            let t285 = t284 * t37;
            let t286 = t39 * t39;
            let t287 = t286 * t39;
            let t288 = f64x8::splat(1.0) / t287;
            let t290 = -t285 * t288 + f64x8::splat(1.0);
            let t292 = t283 * t290 - t177 + t34 - t90 - t92;
            let t293 = t240 * t292;
            let tzk0 = -t34 + t90 + t92 + t177 + t293;
            acc_zk = tzk0;
            let t295 = f64x8::splat(1.0) / t9 / t8;
            let t296 = t7 * t295;
            let t298 = t5 * t296 * t32;
            let t299 = f64x8::splat(0.0011073470983333333) * t298;
            let t300 = t28 * t28;
            let t301 = f64x8::splat(1.0) / t300;
            let t302 = t14 * t301;
            let t304 = f64x8::splat(1.0) / t15 * t2;
            let t305 = t4 * t7;
            let t306 = t305 * t295;
            let t307 = t304 * t306;
            let t309 = t5 * t296;
            let t311 = ((t12).sqrt());
            let t312 = t311 * t2;
            let t313 = t312 * t306;
            let t318 = t22 * t6 / t23 / t8;
            let t320 = -f64x8::splat(0.632975) * t307 - f64x8::splat(0.29896666666666666) * t309 - f64x8::splat(0.1023875) * t313 - f64x8::splat(0.08215666666666667) * t318;
            let t321 = f64x8::splat(1.0) / t31;
            let t322 = t320 * t321;
            let t323 = t302 * t322;
            let t324 = f64x8::splat(1.0) * t323;
            let t325 = t36 * t35;
            let t326 = t325 * t40;
            let t327 = t326 * t89;
            let t328 = f64x8::splat(4.0) * t327;
            let t329 = f64x8::splat(1.0) / t263;
            let t330 = t37 * t329;
            let t331 = t330 * t89;
            let t332 = f64x8::splat(4.0) * t331;
            let t333 = f64x8::splat(1.0) / t38;
            let t334 = t35 * t333;
            let t335 = t42 - t334;
            let t338 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t335));
            let t339 = -t335;
            let t342 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t339));
            let t343 = t338 + t342;
            let t344 = t343 * t60;
            let t345 = t344 * t88;
            let t346 = t41 * t345;
            let t350 = t68 * t68;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t63 * t351;
            let t357 = -f64x8::splat(1.176575) * t307 - f64x8::splat(0.516475) * t309 - f64x8::splat(0.2103875) * t313 - f64x8::splat(0.104195) * t318;
            let t358 = f64x8::splat(1.0) / t71;
            let t359 = t357 * t358;
            let t365 = t81 * t81;
            let t366 = f64x8::splat(1.0) / t365;
            let t367 = t76 * t366;
            let t372 = -f64x8::splat(0.8630833333333333) * t307 - f64x8::splat(0.301925) * t309 - f64x8::splat(0.05501625) * t313 - f64x8::splat(0.082785) * t318;
            let t373 = f64x8::splat(1.0) / t84;
            let t374 = t372 * t373;
            let t377 = f64x8::splat(0.0005323764196666666) * t5 * t296 * t72 + f64x8::splat(1.0) * t352 * t359 - t299 - t324 + f64x8::splat(0.00018311447306006544) * t5 * t296 * t85 + f64x8::splat(0.5848223622634646) * t367 * t374;
            let t378 = t61 * t377;
            let t379 = t41 * t378;
            let t380 = t344 * t86;
            let t381 = f64x8::splat(0.0197516734986138) * t380;
            let t382 = t61 * t2;
            let t384 = t305 * t295 * t85;
            let t385 = t382 * t384;
            let t386 = f64x8::splat(0.00018311447306006544) * t385;
            let t387 = t61 * t76;
            let t389 = t366 * t372 * t373;
            let t390 = t387 * t389;
            let t391 = f64x8::splat(0.5848223622634646) * t390;
            let t392 = t105 * t175;
            let t393 = f64x8::splat(1.0) / t48;
            let t396 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t393 * t335));
            let t397 = f64x8::splat(1.0) / t53;
            let t400 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t397 * t339));
            let t402 = t396 / f64x8::splat(2.0) + t400 / f64x8::splat(2.0);
            let t404 = t97 * t392 * t402;
            let t405 = f64x8::splat(3.0) * t404;
            let t407 = f64x8::splat(1.0) / t139 / t138;
            let t408 = t38 * t8;
            let t410 = f64x8::splat(1.0) / t23 / t408;
            let t411 = t410 * t111;
            let t413 = t121 * t124;
            let t414 = t57 * t130;
            let t415 = t413 * t414;
            let t417 = f64x8::splat(0.005487874319112926) * t411 * t113 * t415;
            let t418 = t108 * t144;
            let t419 = t418 * t122;
            let t420 = t124 * t410;
            let t423 = f64x8::splat(0.009757440539382782) * t419 * t420 * t414;
            let t424 = t112 * t113;
            let t425 = t149 * t124;
            let t427 = t424 * t425 * t128;
            let t428 = t414 * t20;
            let t429 = t132 * t6;
            let t431 = (t299 + t324 + t328 - t332 + t346 + t379 + t381 - t386 - t391) * t113;
            let t433 = t95 * t159;
            let t434 = t433 * t402;
            let t437 = f64x8::splat(3.0) * t115 * t434 - t117 * t431;
            let t438 = t437 * t119;
            let t439 = t429 * t438;
            let t440 = t428 * t439;
            let t444 = f64x8::splat(1.0) / t9 / t408;
            let t445 = t444 * t57;
            let t447 = t445 * t130 * t134;
            let t449 = f64x8::splat(0.1280504007793016) * t126 * t447;
            let t451 = t424 * t413 * t128;
            let t452 = t57 * t116;
            let t453 = t452 * t20;
            let t454 = t429 * t402;
            let t455 = t453 * t454;
            let t458 = -t417 + t423 - f64x8::splat(0.054878743191129266) * t427 * t440 - t449 - f64x8::splat(0.10975748638225853) * t451 * t455;
            let t462 = f64x8::splat(1.0) / t168 / t167;
            let t463 = t418 * t152;
            let t464 = t39 * t38;
            let t465 = f64x8::splat(1.0) / t464;
            let t466 = t465 * t156;
            let t467 = t466 * t159;
            let t468 = t467 * t134;
            let t470 = f64x8::splat(0.0007529191135594791) * t463 * t468;
            let t471 = t143 * t110;
            let t472 = f64x8::splat(1.0) / t471;
            let t473 = t142 * t472;
            let t474 = t473 * t152;
            let t476 = f64x8::splat(0.001338690183908754) * t474 * t468;
            let t477 = t145 * t147;
            let t478 = t148 * t120;
            let t479 = f64x8::splat(1.0) / t478;
            let t480 = t479 * t151;
            let t482 = t477 * t480 * t155;
            let t483 = t156 * t159;
            let t484 = t483 * t2;
            let t485 = t161 * t7;
            let t487 = t484 * t485 * t438;
            let t491 = f64x8::splat(1.0) / t23 / t263;
            let t492 = t491 * t156;
            let t493 = t492 * t159;
            let t494 = t493 * t163;
            let t496 = f64x8::splat(0.052704337949163536) * t153 * t494;
            let t497 = t149 * t151;
            let t499 = t477 * t497 * t155;
            let t500 = t158 * t104;
            let t501 = f64x8::splat(1.0) / t500;
            let t502 = t156 * t501;
            let t503 = t502 * t2;
            let t505 = t503 * t485 * t402;
            let t508 = -t470 + t476 - f64x8::splat(0.022587573406784373) * t482 * t487 - t496 - f64x8::splat(0.045175146813568746) * t499 * t505;
            let t511 = t407 * t458 / f64x8::splat(8.0) + t462 * t508 / f64x8::splat(16.0);
            let t517 = f64x8::splat(1.0) * t511 * t120 + f64x8::splat(1.0) * t171 * t437 * t119;
            let t519 = f64x8::splat(1.0) / t174;
            let t521 = t97 * t106 * t517 * t519;
            let t522 = v_rho0 * v_rho0;
            let t524 = f64x8::splat(1.0) / t179 / t522;
            let t525 = v_tau0 * t524;
            let t528 = t335 / f64x8::splat(2.0);
            let t529 = t185 * t528;
            let t532 = -t528;
            let t533 = t195 * t532;
            let t536 = t420 / f64x8::splat(3.0);
            let t538 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t525 * t186 + f64x8::splat(5.0) / f64x8::splat(3.0) * t182 * t529 + f64x8::splat(5.0) / f64x8::splat(3.0) * t192 * t533 + t536) * t203;
            let t540 = t208 * t208;
            let t541 = f64x8::splat(1.0) / t540;
            let t542 = t207 * t541;
            let t544 = f64x8::splat(5.0) / f64x8::splat(3.0) * t529 + f64x8::splat(5.0) / f64x8::splat(3.0) * t533;
            let t545 = t542 * t544;
            let t548 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t204 * t545 + f64x8::splat(5.0) / f64x8::splat(9.0) * t538 * t210;
            let t549 = ((t219).select(t548, f64x8::splat(0.0)));
            let t552 = t221 * t221;
            let t553 = f64x8::splat(1.0) / t552;
            let t554 = t220 * t553;
            let t557 = -f64x8::splat(1.131) * t549 * t222 - f64x8::splat(1.131) * t554 * t549;
            let t558 = t557 * t225;
            let t559 = ((t218).select(f64x8::splat(0.0), t558));
            let t560 = t234 * t234;
            let t561 = f64x8::splat(1.0) / t560;
            let t562 = ((t232).select(f64x8::splat(0.0), t548));
            let t566 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t562 * t237));
            let t567 = ((t213).select(t559, t566));
            let t568 = t567 * t292;
            let t569 = t243 * t243;
            let t570 = f64x8::splat(1.0) / t569;
            let t573 = -f64x8::splat(0.005519166666666667) * t307 - f64x8::splat(0.013893333333333334) * t309;
            let t574 = t570 * t573;
            let t576 = t247 * t273;
            let t581 = f64x8::splat(1.0) / t255 / t254 * t203;
            let t582 = t581 * t207;
            let t588 = f64x8::splat(1.0) / t270 / t269 * t258;
            let t589 = t588 * t260;
            let t591 = f64x8::splat(1.0) / t9 / t464;
            let t595 = -f64x8::splat(0.014225094736250906) * t582 * t250 * t410 - f64x8::splat(0.001517649901914849) * t589 * t262 * t591;
            let t597 = -f64x8::splat(1.0) * t574 * t576 + t248 * t595;
            let t598 = f64x8::splat(1.0) / t275;
            let t601 = f64x8::splat(0.030197) * t574 + f64x8::splat(0.030197) * t597 * t598;
            let t602 = t601 * t282;
            let t603 = t602 * t290;
            let t604 = t278 * t58;
            let t605 = t344 * t290;
            let t608 = t284 * t325;
            let t609 = t608 * t288;
            let t610 = t286 * t263;
            let t611 = f64x8::splat(1.0) / t610;
            let t612 = t285 * t611;
            let t614 = -f64x8::splat(12.0) * t609 + f64x8::splat(12.0) * t612;
            let t616 = t603 - f64x8::splat(2.363) * t604 * t605 + t283 * t614 - t299 - t324 - t328 + t332 - t346 - t379 - t381 + t386 + t391 - t405 - t521;
            let t617 = t240 * t616;
            let t618 = t299 + t324 + t328 - t332 + t346 + t379 + t381 - t386 - t391 + t405 + t521 + t568 + t617;
            let tvrho0 = t618 * t8 + t177 + t293 - t34 + t90 + t92;
            acc_vrho_0 = tvrho0;
            let t620 = -t42 - t334;
            let t623 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t620));
            let t624 = -t620;
            let t627 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t624));
            let t628 = t623 + t627;
            let t629 = t628 * t60;
            let t630 = t629 * t88;
            let t631 = t41 * t630;
            let t632 = t629 * t86;
            let t633 = f64x8::splat(0.0197516734986138) * t632;
            let t636 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t393 * t620));
            let t639 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t397 * t624));
            let t641 = t636 / f64x8::splat(2.0) + t639 / f64x8::splat(2.0);
            let t643 = t97 * t392 * t641;
            let t644 = f64x8::splat(3.0) * t643;
            let t646 = (t299 + t324 - t328 - t332 + t631 + t379 + t633 - t386 - t391) * t113;
            let t648 = t433 * t641;
            let t651 = f64x8::splat(3.0) * t115 * t648 - t117 * t646;
            let t652 = t651 * t119;
            let t653 = t429 * t652;
            let t654 = t428 * t653;
            let t657 = t429 * t641;
            let t658 = t453 * t657;
            let t661 = -t417 + t423 - f64x8::splat(0.054878743191129266) * t427 * t654 - t449 - f64x8::splat(0.10975748638225853) * t451 * t658;
            let t665 = t484 * t485 * t652;
            let t669 = t503 * t485 * t641;
            let t672 = -t470 + t476 - f64x8::splat(0.022587573406784373) * t482 * t665 - t496 - f64x8::splat(0.045175146813568746) * t499 * t669;
            let t675 = t407 * t661 / f64x8::splat(8.0) + t462 * t672 / f64x8::splat(16.0);
            let t678 = t171 * t651;
            let t681 = f64x8::splat(1.0) * t675 * t120 + f64x8::splat(1.0) * t678 * t119;
            let t684 = t97 * t106 * t681 * t519;
            let t685 = t620 / f64x8::splat(2.0);
            let t686 = t185 * t685;
            let t689 = v_rho1 * v_rho1;
            let t691 = f64x8::splat(1.0) / t189 / t689;
            let t692 = v_tau1 * t691;
            let t695 = -t685;
            let t696 = t195 * t695;
            let t700 = (f64x8::splat(5.0) / f64x8::splat(3.0) * t182 * t686 - f64x8::splat(5.0) / f64x8::splat(3.0) * t692 * t196 + f64x8::splat(5.0) / f64x8::splat(3.0) * t192 * t696 + t536) * t203;
            let t703 = f64x8::splat(5.0) / f64x8::splat(3.0) * t686 + f64x8::splat(5.0) / f64x8::splat(3.0) * t696;
            let t704 = t542 * t703;
            let t707 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t204 * t704 + f64x8::splat(5.0) / f64x8::splat(9.0) * t700 * t210;
            let t708 = ((t219).select(t707, f64x8::splat(0.0)));
            let t713 = -f64x8::splat(1.131) * t708 * t222 - f64x8::splat(1.131) * t554 * t708;
            let t714 = t713 * t225;
            let t715 = ((t218).select(f64x8::splat(0.0), t714));
            let t716 = ((t232).select(f64x8::splat(0.0), t707));
            let t720 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t716 * t237));
            let t721 = ((t213).select(t715, t720));
            let t722 = t721 * t292;
            let t723 = t629 * t290;
            let t727 = f64x8::splat(12.0) * t609 + f64x8::splat(12.0) * t612;
            let t729 = t603 - f64x8::splat(2.363) * t604 * t723 + t283 * t727 - t299 - t324 + t328 + t332 - t631 - t379 - t633 + t386 + t391 - t644 - t684;
            let t730 = t240 * t729;
            let t731 = t299 + t324 - t328 - t332 + t631 + t379 + t633 - t386 - t391 + t644 + t684 + t722 + t730;
            let tvrho1 = t731 * t8 + t177 + t293 - t34 + t90 + t92;
            acc_vrho_1 = tvrho1;
            let t733 = t94 * t106;
            let t734 = t407 * t108;
            let t735 = t111 * t113;
            let t737 = t734 * t735 * t121;
            let t738 = t737 * t135;
            let t740 = t462 * t142;
            let t741 = t740 * t144;
            let t742 = t150 * t124;
            let t743 = t741 * t742;
            let t744 = t743 * t164;
            let t746 = f64x8::splat(0.006859842898891158) * t738 + f64x8::splat(0.0014117233379240233) * t744;
            let t747 = t746 * t120;
            let t748 = t747 * t519;
            let t750 = f64x8::splat(0.10132118364233778) * t733 * t748;
            let t751 = t199 * t203;
            let t752 = t751 * t210;
            let t753 = f64x8::splat(5.0) / f64x8::splat(72.0) * t752;
            let t754 = ((t219).select(-t753, f64x8::splat(0.0)));
            let t759 = -f64x8::splat(1.131) * t754 * t222 - f64x8::splat(1.131) * t554 * t754;
            let t760 = t759 * t225;
            let t761 = ((t218).select(f64x8::splat(0.0), t760));
            let t762 = ((t232).select(f64x8::splat(0.0), -t753));
            let t766 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t762 * t237));
            let t767 = ((t213).select(t761, t766));
            let t768 = t767 * t292;
            let t769 = t207 * t156;
            let t771 = t581 * t769 * t199;
            let t773 = t57 * t124;
            let t775 = t589 * t773 * t265;
            let t777 = f64x8::splat(0.00533441052609409) * t771 + f64x8::splat(0.0005691187132180684) * t775;
            let t778 = t248 * t777;
            let t779 = t598 * t282;
            let t780 = t779 * t290;
            let t783 = f64x8::splat(0.030197) * t778 * t780 - t750;
            let t784 = t240 * t783;
            let tvsigma0 = t8 * (t750 + t768 + t784);
            acc_vsigma_0 = tvsigma0;
            let t788 = f64x8::splat(0.013719685797782316) * t738 + f64x8::splat(0.0028234466758480467) * t744;
            let t789 = t788 * t120;
            let t790 = t789 * t519;
            let t792 = f64x8::splat(0.10132118364233778) * t733 * t790;
            let t793 = f64x8::splat(5.0) / f64x8::splat(36.0) * t752;
            let t794 = ((t219).select(-t793, f64x8::splat(0.0)));
            let t799 = -f64x8::splat(1.131) * t794 * t222 - f64x8::splat(1.131) * t554 * t794;
            let t800 = t799 * t225;
            let t801 = ((t218).select(f64x8::splat(0.0), t800));
            let t802 = ((t232).select(f64x8::splat(0.0), -t793));
            let t806 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t802 * t237));
            let t807 = ((t213).select(t801, t806));
            let t808 = t807 * t292;
            let t811 = f64x8::splat(0.01066882105218818) * t771 + f64x8::splat(0.001138237426436137) * t775;
            let t812 = t248 * t811;
            let t815 = f64x8::splat(0.030197) * t812 * t780 - t792;
            let t816 = t240 * t815;
            let tvsigma1 = t8 * (t792 + t808 + t816);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t818 = t181 * t186;
            let t819 = t249 * t209;
            let t821 = f64x8::splat(5.0) / f64x8::splat(9.0) * t818 * t819;
            let t822 = ((t219).select(t821, f64x8::splat(0.0)));
            let t827 = -f64x8::splat(1.131) * t822 * t222 - f64x8::splat(1.131) * t554 * t822;
            let t828 = t827 * t225;
            let t829 = ((t218).select(f64x8::splat(0.0), t828));
            let t830 = ((t232).select(f64x8::splat(0.0), t821));
            let t834 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t830 * t237));
            let t835 = ((t213).select(t829, t834));
            let t836 = t8 * t835;
            let tvtau0 = t836 * t292;
            acc_vtau_0 = tvtau0;
            let t837 = t191 * t196;
            let t839 = f64x8::splat(5.0) / f64x8::splat(9.0) * t837 * t819;
            let t840 = ((t219).select(t839, f64x8::splat(0.0)));
            let t845 = -f64x8::splat(1.131) * t840 * t222 - f64x8::splat(1.131) * t554 * t840;
            let t846 = t845 * t225;
            let t847 = ((t218).select(f64x8::splat(0.0), t846));
            let t848 = ((t232).select(f64x8::splat(0.0), t839));
            let t852 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t561 * t848 * t237));
            let t853 = ((t213).select(t847, t852));
            let t854 = t8 * t853;
            let tvtau1 = t854 * t292;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
