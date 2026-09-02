//! GGA_X_OL2 vxc unpol kernel (rayon backend).
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
pub fn gga_x_ol2_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
    }
}
