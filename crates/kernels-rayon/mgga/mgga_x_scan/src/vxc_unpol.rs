//! MGGA_X_SCAN vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_scan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t20 * t20;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t39 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t40 = t21 * t21;
            let t42 = t23 * t22;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t40 * t43;
            let t45 = v_sigma * v_sigma;
            let t46 = t45 * t27;
            let t47 = t30 * t30;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t20 / t48;
            let t55 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t39 * t21 * t25 * t34));
            let t56 = t50 * t55;
            let t60 = ((f64x8::splat(146.0)).sqrt());
            let t61 = t60 * t21;
            let t62 = t61 * t25;
            let t65 = v_tau * t28;
            let t66 = t31 * v_rho;
            let t67 = f64x8::splat(1.0) / t66;
            let t73 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t65 * t67 - t34 / f64x8::splat(8.0)) * t21 * t25;
            let t74 = f64x8::splat(1.0) - t73;
            let t76 = t74 * t74;
            let t78 = (simd::exp(-t76 / f64x8::splat(2.0)));
            let t81 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t74 * t78 / f64x8::splat(100.0);
            let t82 = t81 * t81;
            let t83 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t82;
            let t88 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t83);
            let t89 = (t73).simd_le(f64x8::splat(1.0));
            let t90 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t93 = t90 / (-t90 + param_c1);
            let t94 = (-t93).simd_lt(t73);
            let t95 = (t73).simd_lt(-t93);
            let t96 = ((t95).select(t73, -t93));
            let t97 = param_c1 * t96;
            let t98 = f64x8::splat(1.0) - t96;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = (simd::exp(-t97 * t99));
            let t102 = ((t94).select(f64x8::splat(0.0), t101));
            let t103 = ((param_d).abs());
            let t106 = (simd::ln(f64x8::splat(f64::EPSILON) / t103));
            let t109 = (-t106 + param_c2) / t106;
            let t110 = (t73).simd_lt(-t109);
            let t111 = ((t110).select(-t109, t73));
            let t112 = f64x8::splat(1.0) - t111;
            let t115 = (simd::exp(param_c2 / t112));
            let t117 = ((t110).select(f64x8::splat(0.0), -param_d * t115));
            let t118 = ((t89).select(t102, t117));
            let t119 = f64x8::splat(1.0) - t118;
            let t122 = t88 * t119 + f64x8::splat(1.174) * t118;
            let t124 = ((f64x8::splat(3.0)).sqrt());
            let t125 = f64x8::splat(1.0) / t23;
            let t126 = t40 * t125;
            let t127 = ((v_sigma).sqrt());
            let t128 = t127 * t27;
            let t130 = f64x8::splat(1.0) / t20 / v_rho;
            let t132 = t126 * t128 * t130;
            let t133 = ((t132).sqrt());
            let t137 = (simd::exp(-f64x8::splat(9.8958) * t124 / t133));
            let t138 = f64x8::splat(1.0) - t137;
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t122 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t31;
            let t148 = param_k1 * param_k1;
            let t149 = t83 * t83;
            let t151 = t148 / t149;
            let t152 = t30 * v_rho;
            let t154 = f64x8::splat(1.0) / t31 / t152;
            let t155 = t29 * t154;
            let t158 = t47 * t30;
            let t160 = f64x8::splat(1.0) / t20 / t158;
            let t161 = t160 * t55;
            let t165 = t39 * t39;
            let t166 = t22 * t22;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t169 = t45 * v_sigma;
            let t170 = t47 * t47;
            let t171 = t170 * v_rho;
            let t172 = f64x8::splat(1.0) / t171;
            let t182 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t65 * t33 + t155 / f64x8::splat(3.0);
            let t184 = t26 * t78;
            let t187 = t60 * t76;
            let t191 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t62 * t155 - t60 * t182 * t184 / f64x8::splat(180.0) + t187 * t182 * t184 / f64x8::splat(180.0);
            let t194 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t155 - t44 * t46 * t161 / f64x8::splat(54.0) + f64x8::splat(3.0) / f64x8::splat(80.0) * t168 * t169 * t172 * t55 + f64x8::splat(2.0) * t81 * t191;
            let t195 = t194 * t119;
            let t197 = t182 * t21;
            let t199 = f64x8::splat(5.0) / f64x8::splat(9.0) * t197 * t25;
            let t200 = ((t95).select(t199, f64x8::splat(0.0)));
            let t203 = t98 * t98;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t204 * t200;
            let t207 = -param_c1 * t200 * t99 - t97 * t205;
            let t208 = t207 * t101;
            let t209 = ((t94).select(f64x8::splat(0.0), t208));
            let t210 = param_d * param_c2;
            let t211 = t112 * t112;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = ((t110).select(f64x8::splat(0.0), t199));
            let t217 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t213 * t115));
            let t218 = ((t89).select(t209, t217));
            let t221 = t151 * t195 - t88 * t218 + f64x8::splat(1.174) * t218;
            let t226 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t227 = t226 * t226;
            let t228 = t227 * t227;
            let t230 = t228 * t226 * t18;
            let t231 = f64x8::splat(1.0) / t30;
            let t232 = t231 * t122;
            let t234 = f64x8::splat(1.0) / t133 / t132;
            let t236 = t230 * t232 * t234;
            let t238 = t126 * t128 * t137;
            let t242 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t122 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t221 * t138 - f64x8::splat(1.6891736332904388) * t236 * t238));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t242 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t245 = t28 * t33;
            let t246 = t245 * t26;
            let t248 = v_sigma * t27;
            let t252 = f64x8::splat(1.0) / t170;
            let t257 = t25 * t28;
            let t261 = t60 * t28;
            let t262 = t261 * t33;
            let t263 = t262 * t184;
            let t265 = t187 * t28;
            let t267 = t25 * t78;
            let t269 = t265 * t33 * t21 * t267;
            let t271 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t61 * t257 * t33 + t263 / f64x8::splat(1440.0) - t269 / f64x8::splat(1440.0);
            let t274 = f64x8::splat(5.0) / f64x8::splat(972.0) * t246 + t44 * t248 * t56 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(640.0) * t168 * t45 * t252 * t55 + f64x8::splat(2.0) * t81 * t271;
            let t275 = t274 * t119;
            let t277 = f64x8::splat(5.0) / f64x8::splat(72.0) * t246;
            let t278 = ((t95).select(-t277, f64x8::splat(0.0)));
            let t279 = param_c1 * t278;
            let t281 = t204 * t278;
            let t283 = -t279 * t99 - t97 * t281;
            let t284 = t283 * t101;
            let t285 = ((t94).select(f64x8::splat(0.0), t284));
            let t286 = ((t110).select(f64x8::splat(0.0), -t277));
            let t290 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t286 * t115));
            let t291 = ((t89).select(t285, t290));
            let t294 = t151 * t275 - t88 * t291 + f64x8::splat(1.174) * t291;
            let t299 = f64x8::splat(1.0) / v_rho;
            let t300 = t299 * t122;
            let t302 = t230 * t300 * t234;
            let t303 = f64x8::splat(1.0) / t127;
            let t306 = t126 * t303 * t27 * t137;
            let t310 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t294 * t138 + f64x8::splat(0.6334401124839145) * t302 * t306));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t310;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t312 = t261 * t67;
            let t318 = t265 * t67 * t21 * t267 / f64x8::splat(180.0) - t312 * t184 / f64x8::splat(180.0);
            let t319 = t81 * t318;
            let t323 = t28 * t67;
            let t325 = f64x8::splat(5.0) / f64x8::splat(9.0) * t323 * t26;
            let t326 = ((t95).select(t325, f64x8::splat(0.0)));
            let t327 = param_c1 * t326;
            let t331 = -t97 * t204 * t326 - t327 * t99;
            let t332 = t331 * t101;
            let t333 = ((t94).select(f64x8::splat(0.0), t332));
            let t334 = ((t110).select(f64x8::splat(0.0), t325));
            let t338 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t334 * t115));
            let t339 = ((t89).select(t333, t338));
            let t342 = f64x8::splat(2.0) * t151 * t319 * t119 - t88 * t339 + f64x8::splat(1.174) * t339;
            let t347 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t342 * t138));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t347;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
