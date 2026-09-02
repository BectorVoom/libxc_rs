//! MGGA_X_RSCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphar = f64x8::splat(param_alphar);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let param_taur = f64x8::splat(param_taur);
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
            let t65 = t12 * t12;
            let t66 = t65 * t65;
            let t67 = t66 * t12;
            let t68 = t67 * t48;
            let t69 = v_tau * t28;
            let t70 = t31 * v_rho;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = t69 * t71 - t34 / f64x8::splat(8.0);
            let t75 = (f64x8::splat(0.0)).simd_lt(t74);
            let t76 = ((t75).select(t74, f64x8::splat(0.0)));
            let t77 = t76 * t76;
            let t78 = t77 * t76;
            let t79 = t12 * v_rho;
            let t80 = (simd::cbrt(t79));
            let t81 = t80 * t80;
            let t84 = t40 * t24;
            let t88 = f64x8::splat(3.0) / f64x8::splat(40.0) * t27 * t81 * t79 * t84 + param_taur / f64x8::splat(2.0);
            let t89 = t88 * t88;
            let t90 = t89 * t88;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = t65 * t12;
            let t94 = t30 * v_rho;
            let t96 = t80 * t93 * t94;
            let t97 = t28 * t96;
            let t98 = f64x8::splat(1.0) / t89;
            let t99 = t77 * t98;
            let t102 = t97 * t99 / f64x8::splat(16.0) + param_alphar;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t78 * t91 * t103;
            let t106 = t68 * t104 / f64x8::splat(32.0);
            let t107 = f64x8::splat(1.0) - t106;
            let t109 = t107 * t107;
            let t111 = (simd::exp(-t109 / f64x8::splat(2.0)));
            let t114 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t107 * t111 / f64x8::splat(100.0);
            let t115 = t114 * t114;
            let t116 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t115;
            let t121 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t116);
            let t122 = (t106).simd_le(f64x8::splat(2.5));
            let t123 = (f64x8::splat(2.5)).simd_lt(t106);
            let t124 = ((t123).select(f64x8::splat(2.5), t106));
            let t126 = t124 * t124;
            let t128 = t126 * t124;
            let t130 = t126 * t126;
            let t132 = t130 * t124;
            let t134 = t130 * t126;
            let t139 = ((t123).select(t106, f64x8::splat(2.5)));
            let t140 = f64x8::splat(1.0) - t139;
            let t143 = (simd::exp(param_c2 / t140));
            let t145 = ((t122).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t124 - f64x8::splat(0.4445555) * t126 - f64x8::splat(0.663086601049) * t128 + f64x8::splat(1.45129704449) * t130 - f64x8::splat(0.887998041597) * t132 + f64x8::splat(0.234528941479) * t134 - f64x8::splat(0.023185843322) * t130 * t128, -param_d * t143));
            let t146 = f64x8::splat(1.0) - t145;
            let t149 = t121 * t146 + f64x8::splat(1.174) * t145;
            let t151 = ((f64x8::splat(3.0)).sqrt());
            let t152 = f64x8::splat(1.0) / t23;
            let t153 = t40 * t152;
            let t154 = ((v_sigma).sqrt());
            let t155 = t154 * t27;
            let t157 = f64x8::splat(1.0) / t20 / v_rho;
            let t159 = t153 * t155 * t157;
            let t160 = ((t159).sqrt());
            let t164 = (simd::exp(-f64x8::splat(9.8958) * t151 / t160));
            let t165 = f64x8::splat(1.0) - t164;
            let t169 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t149 * t165));
            let tzk0 = f64x8::splat(2.0) * t169;
            acc_zk = tzk0;
            let t170 = f64x8::splat(1.0) / t31;
            let t175 = param_k1 * param_k1;
            let t176 = t116 * t116;
            let t178 = t175 / t176;
            let t180 = f64x8::splat(1.0) / t31 / t94;
            let t181 = t29 * t180;
            let t184 = t47 * t30;
            let t186 = f64x8::splat(1.0) / t20 / t184;
            let t187 = t186 * t55;
            let t191 = t39 * t39;
            let t192 = t22 * t22;
            let t193 = f64x8::splat(1.0) / t192;
            let t194 = t191 * t193;
            let t195 = t45 * v_sigma;
            let t196 = t47 * t47;
            let t197 = t196 * v_rho;
            let t198 = f64x8::splat(1.0) / t197;
            let t205 = t67 * t47;
            let t208 = t68 * t77;
            let t209 = t91 * t103;
            let t214 = ((t75).select(-f64x8::splat(5.0) / f64x8::splat(3.0) * t69 * t33 + t181 / f64x8::splat(3.0), f64x8::splat(0.0)));
            let t215 = t209 * t214;
            let t218 = t66 * t65;
            let t219 = t218 * t48;
            let t220 = t89 * t89;
            let t221 = f64x8::splat(1.0) / t220;
            let t222 = t78 * t221;
            let t224 = t103 * t27;
            let t226 = t81 * t40 * t24;
            let t227 = t224 * t226;
            let t230 = t68 * t78;
            let t231 = t102 * t102;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t91 * t232;
            let t235 = t80 * t65 * t30;
            let t236 = t28 * t235;
            let t240 = t76 * t98;
            let t244 = t205 * t77;
            let t246 = t91 * t40 * t24;
            let t249 = f64x8::splat(5.0) / f64x8::splat(24.0) * t236 * t99 * t12 + t97 * t240 * t214 / f64x8::splat(8.0) - t244 * t246 / f64x8::splat(32.0);
            let t250 = t233 * t249;
            let t253 = -f64x8::splat(5.0) / f64x8::splat(32.0) * t205 * t104 - f64x8::splat(3.0) / f64x8::splat(32.0) * t208 * t215 + f64x8::splat(3.0) / f64x8::splat(256.0) * t219 * t222 * t227 + t230 * t250 / f64x8::splat(32.0);
            let t257 = t60 * t109;
            let t258 = t253 * t111;
            let t261 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t62 * t181 + t60 * t253 * t111 / f64x8::splat(100.0) - t257 * t258 / f64x8::splat(100.0);
            let t264 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t181 - t44 * t46 * t187 / f64x8::splat(54.0) + f64x8::splat(3.0) / f64x8::splat(80.0) * t194 * t195 * t198 * t55 + f64x8::splat(2.0) * t114 * t261;
            let t265 = t264 * t146;
            let t267 = -t253;
            let t268 = ((t123).select(f64x8::splat(0.0), t267));
            let t270 = t124 * t268;
            let t272 = t126 * t268;
            let t274 = t128 * t268;
            let t276 = t130 * t268;
            let t278 = t132 * t268;
            let t283 = param_d * param_c2;
            let t284 = t140 * t140;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = ((t123).select(t267, f64x8::splat(0.0)));
            let t290 = ((t122).select(-f64x8::splat(0.667) * t268 - f64x8::splat(0.889111) * t270 - f64x8::splat(1.989259803147) * t272 + f64x8::splat(5.80518817796) * t274 - f64x8::splat(4.439990207985) * t276 + f64x8::splat(1.407173648874) * t278 - f64x8::splat(0.162300903254) * t134 * t268, -t283 * t285 * t286 * t143));
            let t293 = t178 * t265 - t121 * t290 + f64x8::splat(1.174) * t290;
            let t298 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t299 = t298 * t298;
            let t300 = t299 * t299;
            let t302 = t300 * t298 * t18;
            let t303 = f64x8::splat(1.0) / t30;
            let t304 = t303 * t149;
            let t306 = f64x8::splat(1.0) / t160 / t159;
            let t308 = t302 * t304 * t306;
            let t310 = t153 * t155 * t164;
            let t314 = ((t3).select(f64x8::splat(0.0), -t19 * t170 * t149 * t165 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t293 * t165 - f64x8::splat(1.6891736332904388) * t308 * t310));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t314 + f64x8::splat(2.0) * t169;
            acc_vrho = tvrho0;
            let t317 = t28 * t33;
            let t320 = v_sigma * t27;
            let t324 = f64x8::splat(1.0) / t196;
            let t329 = t25 * t28;
            let t334 = ((t75).select(-t317 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t335 = t209 * t334;
            let t338 = t77 * t77;
            let t340 = f64x8::splat(1.0) / t220 / t88;
            let t341 = t338 * t340;
            let t342 = t68 * t341;
            let t343 = t232 * t28;
            let t344 = t96 * t334;
            let t345 = t343 * t344;
            let t348 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t208 * t335 + t342 * t345 / f64x8::splat(256.0);
            let t349 = t60 * t348;
            let t352 = t348 * t111;
            let t355 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t61 * t329 * t33 + t349 * t111 / f64x8::splat(100.0) - t257 * t352 / f64x8::splat(100.0);
            let t358 = f64x8::splat(5.0) / f64x8::splat(972.0) * t26 * t317 + t44 * t320 * t56 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(640.0) * t194 * t45 * t324 * t55 + f64x8::splat(2.0) * t114 * t355;
            let t359 = t358 * t146;
            let t361 = -t348;
            let t362 = ((t123).select(f64x8::splat(0.0), t361));
            let t364 = t124 * t362;
            let t366 = t126 * t362;
            let t368 = t128 * t362;
            let t370 = t130 * t362;
            let t372 = t132 * t362;
            let t377 = ((t123).select(t361, f64x8::splat(0.0)));
            let t381 = ((t122).select(-f64x8::splat(0.667) * t362 - f64x8::splat(0.889111) * t364 - f64x8::splat(1.989259803147) * t366 + f64x8::splat(5.80518817796) * t368 - f64x8::splat(4.439990207985) * t370 + f64x8::splat(1.407173648874) * t372 - f64x8::splat(0.162300903254) * t134 * t362, -t283 * t285 * t377 * t143));
            let t384 = t178 * t359 - t121 * t381 + f64x8::splat(1.174) * t381;
            let t389 = f64x8::splat(1.0) / v_rho;
            let t390 = t389 * t149;
            let t392 = t302 * t390 * t306;
            let t393 = f64x8::splat(1.0) / t154;
            let t396 = t153 * t393 * t27 * t164;
            let t400 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t384 * t165 + f64x8::splat(0.6334401124839145) * t392 * t396));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t400;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t403 = ((t75).select(t28 * t71, f64x8::splat(0.0)));
            let t404 = t209 * t403;
            let t407 = t96 * t403;
            let t408 = t343 * t407;
            let t411 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t208 * t404 + t342 * t408 / f64x8::splat(256.0);
            let t412 = t60 * t411;
            let t414 = t411 * t111;
            let t417 = t412 * t111 / f64x8::splat(100.0) - t257 * t414 / f64x8::splat(100.0);
            let t418 = t114 * t417;
            let t422 = -t411;
            let t423 = ((t123).select(f64x8::splat(0.0), t422));
            let t425 = t124 * t423;
            let t427 = t126 * t423;
            let t429 = t128 * t423;
            let t431 = t130 * t423;
            let t433 = t132 * t423;
            let t438 = ((t123).select(t422, f64x8::splat(0.0)));
            let t442 = ((t122).select(-f64x8::splat(0.667) * t423 - f64x8::splat(0.889111) * t425 - f64x8::splat(1.989259803147) * t427 + f64x8::splat(5.80518817796) * t429 - f64x8::splat(4.439990207985) * t431 + f64x8::splat(1.407173648874) * t433 - f64x8::splat(0.162300903254) * t134 * t423, -t283 * t285 * t438 * t143));
            let t445 = f64x8::splat(2.0) * t178 * t418 * t146 - t121 * t442 + f64x8::splat(1.174) * t442;
            let t450 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t445 * t165));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t450;
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
