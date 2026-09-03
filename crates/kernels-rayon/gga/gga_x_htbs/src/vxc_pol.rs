//! GGA_X_HTBS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_htbs.c`
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
pub fn gga_x_htbs_vxc_pol(
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t33 = t29 / t31;
            let t34 = ((v_sigma0).sqrt());
            let t35 = (simd::cbrt(v_rho0));
            let t37 = f64x8::splat(1.0) / t35 / v_rho0;
            let t39 = t33 * t34 * t37;
            let t40 = t39 / f64x8::splat(12.0);
            let t41 = (t40).simd_le(f64x8::splat(0.6));
            let t42 = t31 * t31;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t28 * t43;
            let t45 = v_rho0 * v_rho0;
            let t46 = t35 * t35;
            let t48 = f64x8::splat(1.0) / t46 / t45;
            let t49 = v_sigma0 * t48;
            let t50 = t44 * t49;
            let t53 = (simd::exp(-t50 / f64x8::splat(24.0)));
            let t58 = f64x8::splat(1.0) / t31 / t30;
            let t59 = t29 * t58;
            let t60 = v_sigma0 * v_sigma0;
            let t61 = t45 * t45;
            let t62 = t61 * v_rho0;
            let t64 = f64x8::splat(1.0) / t35 / t62;
            let t66 = t59 * t60 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t66;
            let t69 = (simd::ln(t68));
            let t70 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t50 + f64x8::splat(0.004002424276710846) * t44 * t49 * t53 + t69;
            let t73 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t70;
            let t74 = (f64x8::splat(2.6)).simd_le(t40);
            let t76 = (simd::exp(-f64x8::splat(0.011376190545424806) * t50));
            let t78 = f64x8::splat(1.804) - f64x8::splat(0.804) * t76;
            let t79 = f64x8::splat(0.190125) * t39;
            let t80 = f64x8::splat(0.195) * t50;
            let t81 = t34 * v_sigma0;
            let t82 = f64x8::splat(1.0) / t61;
            let t84 = f64x8::splat(0.008812832118890838) * t81 * t82;
            let t85 = f64x8::splat(0.0026041666666666665) * t66;
            let t88 = t28 / t42 / t30;
            let t89 = t34 * t60;
            let t90 = t61 * t45;
            let t92 = f64x8::splat(1.0) / t46 / t90;
            let t95 = f64x8::splat(0.00016276041666666666) * t88 * t89 * t92;
            let t96 = -f64x8::splat(0.40608) + t79 - t80 + t84 - t85 + t95;
            let t98 = f64x8::splat(1.40608) - t79 + t80 - t84 + t85 - t95;
            let t101 = ((t41).select(t73, (t74).select(t78, t98 * t73 + t96 * t78)));
            let t105 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t101));
            let t106 = (v_rho1).simd_le(dens_threshold);
            let t107 = -t16;
            let t109 = ((t14).select(t11, (t10).select(t15, t107 * t7)));
            let t110 = f64x8::splat(1.0) + t109;
            let t111 = (t110).simd_le(zeta_threshold);
            let t112 = (simd::cbrt(t110));
            let t114 = ((t111).select(t22, t112 * t110));
            let t115 = t114 * t26;
            let t116 = ((v_sigma2).sqrt());
            let t117 = (simd::cbrt(v_rho1));
            let t119 = f64x8::splat(1.0) / t117 / v_rho1;
            let t121 = t33 * t116 * t119;
            let t122 = t121 / f64x8::splat(12.0);
            let t123 = (t122).simd_le(f64x8::splat(0.6));
            let t124 = v_rho1 * v_rho1;
            let t125 = t117 * t117;
            let t127 = f64x8::splat(1.0) / t125 / t124;
            let t128 = v_sigma2 * t127;
            let t129 = t44 * t128;
            let t132 = (simd::exp(-t129 / f64x8::splat(24.0)));
            let t136 = v_sigma2 * v_sigma2;
            let t137 = t124 * t124;
            let t138 = t137 * v_rho1;
            let t140 = f64x8::splat(1.0) / t117 / t138;
            let t142 = t59 * t136 * t140;
            let t144 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t142;
            let t145 = (simd::ln(t144));
            let t146 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t129 + f64x8::splat(0.004002424276710846) * t44 * t128 * t132 + t145;
            let t149 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t146;
            let t150 = (f64x8::splat(2.6)).simd_le(t122);
            let t152 = (simd::exp(-f64x8::splat(0.011376190545424806) * t129));
            let t154 = f64x8::splat(1.804) - f64x8::splat(0.804) * t152;
            let t155 = f64x8::splat(0.190125) * t121;
            let t156 = f64x8::splat(0.195) * t129;
            let t157 = t116 * v_sigma2;
            let t158 = f64x8::splat(1.0) / t137;
            let t160 = f64x8::splat(0.008812832118890838) * t157 * t158;
            let t161 = f64x8::splat(0.0026041666666666665) * t142;
            let t162 = t116 * t136;
            let t163 = t137 * t124;
            let t165 = f64x8::splat(1.0) / t125 / t163;
            let t168 = f64x8::splat(0.00016276041666666666) * t88 * t162 * t165;
            let t169 = -f64x8::splat(0.40608) + t155 - t156 + t160 - t161 + t168;
            let t171 = f64x8::splat(1.40608) - t155 + t156 - t160 + t161 - t168;
            let t174 = ((t123).select(t149, (t150).select(t154, t171 * t149 + t169 * t154)));
            let t178 = ((t106).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t115 * t174));
            let tzk0 = t105 + t178;
            acc_zk = tzk0;
            let t179 = t6 * t6;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t16 * t180;
            let t183 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t181)));
            let t186 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t183));
            let t187 = t186 * t26;
            let t191 = t26 * t26;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t25 * t192;
            let t196 = t5 * t193 * t101 / f64x8::splat(8.0);
            let t197 = t70 * t70;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t45 * v_rho0;
            let t201 = f64x8::splat(1.0) / t46 / t199;
            let t202 = v_sigma0 * t201;
            let t203 = t44 * t202;
            let t209 = f64x8::splat(1.0) / t35 / t90;
            let t210 = t60 * t209;
            let t214 = f64x8::splat(1.0) / t68;
            let t218 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t203 - f64x8::splat(0.010673131404562256) * t44 * t202 * t53 + f64x8::splat(0.00044471380852342736) * t59 * t210 * t53 - f64x8::splat(7.349508643668351e-05) * t59 * t210 * t214;
            let t221 = t202 * t76;
            let t225 = f64x8::splat(1.0) / t35 / t45;
            let t230 = f64x8::splat(1.0) / t62;
            let t235 = t61 * t199;
            let t237 = f64x8::splat(1.0) / t46 / t235;
            let t241 = -f64x8::splat(0.2535) * t33 * t34 * t225 + f64x8::splat(0.52) * t203 - f64x8::splat(0.03525132847556335) * t81 * t230 + f64x8::splat(0.013888888888888888) * t59 * t210 - f64x8::splat(0.0010850694444444445) * t88 * t89 * t237;
            let t243 = t96 * t28;
            let t244 = t243 * t43;
            let t247 = -t241;
            let t249 = t98 * t198;
            let t253 = ((t41).select(f64x8::splat(0.646416) * t198 * t218, (t74).select(-f64x8::splat(0.024390552529390784) * t44 * t221, t241 * t78 - f64x8::splat(0.024390552529390784) * t244 * t221 + t247 * t73 + f64x8::splat(0.646416) * t249 * t218)));
            let t258 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t187 * t101 - t196 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t253));
            let t259 = t107 * t180;
            let t261 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t259)));
            let t264 = ((t111).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t112 * t261));
            let t265 = t264 * t26;
            let t269 = t114 * t192;
            let t272 = t5 * t269 * t174 / f64x8::splat(8.0);
            let t274 = ((t106).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t265 * t174 - t272));
            let tvrho0 = t105 + t178 + t6 * (t258 + t274);
            acc_vrho_0 = tvrho0;
            let t278 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t181)));
            let t281 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t278));
            let t282 = t281 * t26;
            let t287 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t282 * t101 - t196));
            let t289 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t259)));
            let t292 = ((t111).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t112 * t289));
            let t293 = t292 * t26;
            let t297 = t146 * t146;
            let t298 = f64x8::splat(1.0) / t297;
            let t299 = t124 * v_rho1;
            let t301 = f64x8::splat(1.0) / t125 / t299;
            let t302 = v_sigma2 * t301;
            let t303 = t44 * t302;
            let t309 = f64x8::splat(1.0) / t117 / t163;
            let t310 = t136 * t309;
            let t314 = f64x8::splat(1.0) / t144;
            let t318 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t303 - f64x8::splat(0.010673131404562256) * t44 * t302 * t132 + f64x8::splat(0.00044471380852342736) * t59 * t310 * t132 - f64x8::splat(7.349508643668351e-05) * t59 * t310 * t314;
            let t321 = t302 * t152;
            let t325 = f64x8::splat(1.0) / t117 / t124;
            let t330 = f64x8::splat(1.0) / t138;
            let t335 = t137 * t299;
            let t337 = f64x8::splat(1.0) / t125 / t335;
            let t341 = -f64x8::splat(0.2535) * t33 * t116 * t325 + f64x8::splat(0.52) * t303 - f64x8::splat(0.03525132847556335) * t157 * t330 + f64x8::splat(0.013888888888888888) * t59 * t310 - f64x8::splat(0.0010850694444444445) * t88 * t162 * t337;
            let t343 = t169 * t28;
            let t344 = t343 * t43;
            let t347 = -t341;
            let t349 = t171 * t298;
            let t353 = ((t123).select(f64x8::splat(0.646416) * t298 * t318, (t150).select(-f64x8::splat(0.024390552529390784) * t44 * t321, t341 * t154 - f64x8::splat(0.024390552529390784) * t344 * t321 + t347 * t149 + f64x8::splat(0.646416) * t349 * t318)));
            let t358 = ((t106).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t293 * t174 - t272 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t115 * t353));
            let tvrho1 = t105 + t178 + t6 * (t287 + t358);
            acc_vrho_1 = tvrho1;
            let t361 = t44 * t48;
            let t366 = v_sigma0 * t64;
            let t373 = f64x8::splat(5.0) / f64x8::splat(972.0) * t361 + f64x8::splat(0.004002424276710846) * t44 * t48 * t53 - f64x8::splat(0.00016676767819628525) * t59 * t366 * t53 + f64x8::splat(2.7560657413756314e-05) * t59 * t366 * t214;
            let t379 = f64x8::splat(1.0) / t34;
            let t391 = f64x8::splat(0.0950625) * t33 * t379 * t37 - f64x8::splat(0.195) * t361 + f64x8::splat(0.013219248178336257) * t34 * t82 - f64x8::splat(0.005208333333333333) * t59 * t366 + f64x8::splat(0.0004069010416666667) * t88 * t81 * t92;
            let t394 = t43 * t48 * t76;
            let t397 = -t391;
            let t402 = ((t41).select(f64x8::splat(0.646416) * t198 * t373, (t74).select(f64x8::splat(0.009146457198521543) * t44 * t48 * t76, t391 * t78 + f64x8::splat(0.009146457198521543) * t243 * t394 + t397 * t73 + f64x8::splat(0.646416) * t249 * t373)));
            let t406 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t402));
            let tvsigma0 = t6 * t406;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t407 = t44 * t127;
            let t412 = v_sigma2 * t140;
            let t419 = f64x8::splat(5.0) / f64x8::splat(972.0) * t407 + f64x8::splat(0.004002424276710846) * t44 * t127 * t132 - f64x8::splat(0.00016676767819628525) * t59 * t412 * t132 + f64x8::splat(2.7560657413756314e-05) * t59 * t412 * t314;
            let t425 = f64x8::splat(1.0) / t116;
            let t437 = f64x8::splat(0.0950625) * t33 * t425 * t119 - f64x8::splat(0.195) * t407 + f64x8::splat(0.013219248178336257) * t116 * t158 - f64x8::splat(0.005208333333333333) * t59 * t412 + f64x8::splat(0.0004069010416666667) * t88 * t157 * t165;
            let t440 = t43 * t127 * t152;
            let t443 = -t437;
            let t448 = ((t123).select(f64x8::splat(0.646416) * t298 * t419, (t150).select(f64x8::splat(0.009146457198521543) * t44 * t127 * t152, t437 * t154 + f64x8::splat(0.009146457198521543) * t343 * t440 + t443 * t149 + f64x8::splat(0.646416) * t349 * t419)));
            let t452 = ((t106).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t115 * t448));
            let tvsigma2 = t6 * t452;
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
