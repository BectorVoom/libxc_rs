//! GGA_X_OL2 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ol2_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_bb * sigma[ip];
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t30 = rmath::sqrt(sigma[ip]);
        let t31 = param_cc * t30;
        let t33 = 1.0 / t18 / rho[ip];
        let t38 = 4.0 * t30 * t21 * t33 + t21;
        let t39 = 1.0 / t38;
        let t40 = t21 * t33 * t39;
        let t42 = param_aa + 0.013888888888888888 * t20 * t27 + t31 * t40;
        let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        let t48 = t17 / t24;
        let t52 = t23 * rho[ip];
        let t54 = 1.0 / t24 / t52;
        let t55 = t22 * t54;
        let t61 = t21 / t18 / t23 * t39;
        let t64 = param_cc * sigma[ip];
        let t65 = t38 * t38;
        let t66 = 1.0 / t65;
        let t67 = t55 * t66;
        let t70 = -0.037037037037037035 * t20 * t55 - 4.0 / 3.0 * t31 * t61 + 16.0 / 3.0 * t64 * t67;
        let t75 = piecewise3(t2, 0.0, -t6 * t48 * t42 / 8.0 - 3.0 / 8.0 * t6 * t19 * t70);
        let tvrho0 = 2.0 * rho[ip] * t75 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t78 = param_bb * t22;
        let t81 = 1.0 / t30;
        let t82 = param_cc * t81;
        let t85 = param_cc * t22;
        let t89 = 0.013888888888888888 * t78 * t26 + t82 * t40 / 2.0 - 2.0 * t85 * t26 * t66;
        let t93 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
        let t98 = t17 / t24 / rho[ip];
        let t105 = t23 * t23;
        let t107 = 1.0 / t24 / t105;
        let t108 = t22 * t107;
        let t114 = t21 / t18 / t52 * t39;
        let t117 = t108 * t66;
        let t120 = t30 * sigma[ip];
        let t121 = param_cc * t120;
        let t122 = t105 * t23;
        let t123 = 1.0 / t122;
        let t125 = 1.0 / t65 / t38;
        let t126 = t123 * t125;
        let t129 = 0.13580246913580246 * t20 * t108 + 28.0 / 9.0 * t31 * t114 - 80.0 / 3.0 * t64 * t117 + 1024.0 / 9.0 * t121 * t126;
        let t134 = piecewise3(t2, 0.0, t6 * t98 * t42 / 12.0 - t6 * t48 * t70 / 4.0 - 3.0 / 8.0 * t6 * t19 * t129);
        let tv2rho20 = 2.0 * rho[ip] * t134 + 4.0 * t75;
        v2rho2[ip] += tv2rho20;
        let t147 = t105 * rho[ip];
        let t148 = 1.0 / t147;
        let t150 = t125 * t30;
        let t153 = -0.037037037037037035 * t78 * t54 - 2.0 / 3.0 * t82 * t61 + 8.0 * t85 * t54 * t66 - 128.0 / 3.0 * param_cc * t148 * t150;
        let t158 = piecewise3(t2, 0.0, -t6 * t48 * t89 / 8.0 - 3.0 / 8.0 * t6 * t19 * t153);
        let tv2rhosigma0 = 2.0 * rho[ip] * t158 + 2.0 * t93;
        v2rhosigma[ip] += tv2rhosigma0;
        let t161 = 1.0 / t120;
        let t162 = param_cc * t161;
        let t165 = 1.0 / sigma[ip];
        let t166 = param_cc * t165;
        let t167 = t27 * t66;
        let t169 = 1.0 / t105;
        let t174 = -t162 * t40 / 4.0 - t166 * t167 + 16.0 * param_cc * t169 * t125 * t81;
        let t178 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t174);
        let tv2sigma20 = 2.0 * rho[ip] * t178;
        v2sigma2[ip] += tv2sigma20;
    }
}
