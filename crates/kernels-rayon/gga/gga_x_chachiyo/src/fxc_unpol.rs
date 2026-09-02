//! GGA_X_CHACHIYO fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_chachiyo_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = t10 + 1.0;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t4 * t4;
        let t21 = t3 * t20;
        let t22 = rho[ip] * rho[ip];
        let t23 = t19 * t19;
        let t25 = 1.0 / t23 / t22;
        let t29 = M_PI * M_PI;
        let t30 = t3 * t3;
        let t31 = t30 * t4;
        let t32 = rmath::sqrt(sigma[ip]);
        let t34 = 1.0 / t19 / rho[ip];
        let t36 = t31 * t32 * t34;
        let t38 = 2.0 / 27.0 * t36 + 1.0;
        let t39 = rmath::ln(t38);
        let t41 = 4.0 / 81.0 * t21 * sigma[ip] * t25 + t29 * t39;
        let t44 = 2.0 / 9.0 * t36 + t29;
        let t45 = 1.0 / t44;
        let t46 = 1.0 / t39;
        let t47 = t45 * t46;
        let t51 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t41 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = 1.0 / t23;
        let t57 = t22 * rho[ip];
        let t59 = 1.0 / t23 / t57;
        let t64 = t4 * t29 * t30;
        let t66 = 1.0 / t19 / t22;
        let t68 = 1.0 / t38;
        let t72 = -32.0 / 243.0 * t21 * sigma[ip] * t59 - 8.0 / 81.0 * t64 * t32 * t66 * t68;
        let t78 = t17 / t22;
        let t79 = t78 * t41;
        let t80 = t44 * t44;
        let t81 = 1.0 / t80;
        let t82 = t81 * t46;
        let t83 = t82 * t32;
        let t86 = t39 * t39;
        let t87 = 1.0 / t86;
        let t88 = t45 * t87;
        let t90 = t88 * t32 * t68;
        let t94 = piecewise3(t2, 0.0, -t18 * t52 * t41 * t47 / 8.0 - 3.0 / 8.0 * t18 * t19 * t72 * t47 - t79 * t83 / 3.0 - t79 * t90 / 9.0);
        let tvrho0 = 2.0 * rho[ip] * t94 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t99 = 1.0 / t32;
        let t104 = 4.0 / 81.0 * t21 * t25 + t64 * t99 * t34 * t68 / 27.0;
        let t110 = t17 / rho[ip];
        let t111 = t110 * t41;
        let t112 = t82 * t99;
        let t116 = t88 * t99 * t68;
        let t120 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t104 * t47 + t111 * t112 / 8.0 + t111 * t116 / 24.0);
        let tvsigma0 = 2.0 * rho[ip] * t120;
        vsigma[ip] += tvsigma0;
        let t124 = 1.0 / t23 / rho[ip];
        let t134 = t17 / t57;
        let t135 = t134 * t41;
        let t140 = t22 * t22;
        let t142 = 1.0 / t23 / t140;
        let t143 = sigma[ip] * t142;
        let t147 = 1.0 / t19 / t57;
        let t153 = t20 * t29 * t3;
        let t154 = t38 * t38;
        let t155 = 1.0 / t154;
        let t159 = 352.0 / 729.0 * t21 * t143 + 56.0 / 243.0 * t64 * t32 * t147 * t68 - 64.0 / 2187.0 * t153 * t143 * t155;
        let t164 = t78 * t72;
        let t170 = 1.0 / t19 / t140;
        let t171 = t17 * t170;
        let t173 = 1.0 / t80 / t44;
        let t174 = t41 * t173;
        let t175 = t171 * t174;
        let t177 = t46 * sigma[ip] * t31;
        let t180 = t41 * t81;
        let t181 = t171 * t180;
        let t182 = t87 * sigma[ip];
        let t183 = t31 * t68;
        let t184 = t182 * t183;
        let t187 = t41 * t45;
        let t188 = t171 * t187;
        let t190 = 1.0 / t86 / t39;
        let t193 = t155 * t30 * t4;
        let t194 = t190 * sigma[ip] * t193;
        let t197 = t182 * t193;
        let t200 = t18 * t124 * t41 * t47 / 12.0 - t18 * t52 * t72 * t47 / 4.0 + 5.0 / 9.0 * t135 * t83 + 5.0 / 27.0 * t135 * t90 - 3.0 / 8.0 * t18 * t19 * t159 * t47 - 2.0 / 3.0 * t164 * t83 - 2.0 / 9.0 * t164 * t90 - 16.0 / 81.0 * t175 * t177 - 16.0 / 243.0 * t181 * t184 - 16.0 / 729.0 * t188 * t194 - 8.0 / 729.0 * t188 * t197;
        let t201 = piecewise3(t2, 0.0, t200);
        let tv2rho20 = 2.0 * rho[ip] * t201 + 4.0 * t94;
        v2rho2[ip] += tv2rho20;
        let t217 = -32.0 / 243.0 * t21 * t59 - 4.0 / 81.0 * t64 * t99 * t66 * t68 + 8.0 / 729.0 * t153 * t59 * t155;
        let t222 = t78 * t104;
        let t229 = t110 * t72;
        let t232 = t17 * t147;
        let t235 = t173 * t46 * t31;
        let t238 = t232 * t180;
        let t241 = t87 * t30 * t4 * t68;
        let t248 = t232 * t187;
        let t250 = t190 * t155 * t31;
        let t254 = t87 * t155 * t31;
        let t257 = -t18 * t52 * t104 * t47 / 8.0 - 3.0 / 8.0 * t18 * t19 * t217 * t47 - t222 * t83 / 3.0 - t222 * t90 / 9.0 - t79 * t112 / 8.0 + t229 * t112 / 8.0 + 2.0 / 27.0 * t232 * t41 * t235 + 2.0 / 81.0 * t238 * t241 - t79 * t116 / 24.0 + t229 * t116 / 24.0 + 2.0 / 243.0 * t248 * t250 + t248 * t254 / 243.0;
        let t258 = piecewise3(t2, 0.0, t257);
        let tv2rhosigma0 = 2.0 * rho[ip] * t258 + 2.0 * t120;
        v2rhosigma[ip] += tv2rhosigma0;
        let t261 = t32 * sigma[ip];
        let t262 = 1.0 / t261;
        let t267 = 1.0 / sigma[ip];
        let t272 = -t64 * t262 * t34 * t68 / 54.0 - t153 * t267 * t25 * t155 / 243.0;
        let t277 = t110 * t104;
        let t282 = t17 * t66;
        let t283 = t282 * t174;
        let t285 = t46 * t267 * t31;
        let t288 = t282 * t180;
        let t289 = t87 * t267;
        let t290 = t289 * t183;
        let t293 = t82 * t262;
        let t296 = t282 * t187;
        let t297 = t190 * t267;
        let t298 = t297 * t193;
        let t302 = t88 * t262 * t68;
        let t305 = t289 * t193;
        let t309 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t272 * t47 + t277 * t112 / 4.0 + t277 * t116 / 12.0 - t283 * t285 / 36.0 - t288 * t290 / 108.0 - t111 * t293 / 16.0 - t296 * t298 / 324.0 - t111 * t302 / 48.0 - t296 * t305 / 648.0);
        let tv2sigma20 = 2.0 * rho[ip] * t309;
        v2sigma2[ip] += tv2sigma20;
    }
}
