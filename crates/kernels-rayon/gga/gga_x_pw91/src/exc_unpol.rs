//! GGA_X_PW91 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pw91_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = f64::exp(-param_alpha * t20 * t25 * t34 / 24.0);
        let t40 = (param_d * t37 + param_c) * t20;
        let t41 = t40 * t25;
        let t44 = t20 * t20;
        let t45 = 1.0 / t23;
        let t46 = t44 * t45;
        let t47 = f64::sqrt(sigma[ip]);
        let t50 = 1.0 / t18 / rho[ip];
        let t51 = t47 * t27 * t50;
        let t54 = f64::powf(t46 * t51 / 12.0, param_expo);
        let t55 = param_f * t54;
        let t56 = t41 * t34 / 24.0 - t55;
        let t57 = t46 * t47;
        let t63 = f64::ln(param_b * t44 * t45 * t51 / 12.0 + f64::sqrt(pow_2(param_b * t44 * t45 * t51 / 12.0) + 1.0));
        let t64 = param_a * t63;
        let t65 = t27 * t50 * t64;
        let t68 = 1.0 + t57 * t65 / 12.0 + t55;
        let t69 = 1.0 / t68;
        let t71 = t56 * t69 + 1.0;
        let t75 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t71);
        let tzk0 = 2.0 * t75;
        zk[ip] += tzk0;
    }
}
