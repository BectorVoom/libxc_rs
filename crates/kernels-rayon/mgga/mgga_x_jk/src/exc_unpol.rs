//! MGGA_X_JK exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_jk_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = t4 * t4;
        let t22 = param_beta * t21;
        let t24 = pow_1_3(1.0 / M_PI);
        let t25 = 1.0 / t24;
        let t26 = M_CBRT4;
        let t27 = t25 * t26;
        let t28 = t22 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t34 = t33 * t32;
        let t35 = 1.0 / t34;
        let t36 = param_gamma * param_beta;
        let t37 = f64::sqrt(sigma[ip]);
        let t38 = t36 * t37;
        let t40 = 1.0 / t19 / rho[ip];
        let t41 = t29 * t40;
        let t44 = f64::ln(t37 * t29 * t40 + f64::sqrt(pow_2(t37 * t29 * t40) + 1.0));
        let t45 = t41 * t44;
        let t47 = t38 * t45 + 1.0;
        let t48 = 1.0 / t47;
        let t49 = t35 * t48;
        let t50 = t31 * t35;
        let t51 = lapl[ip] * t30;
        let t52 = t33 * rho[ip];
        let t53 = 1.0 / t52;
        let t55 = -t51 * t53 + t50;
        let t56 = 1.0 / sigma[ip];
        let t57 = t55 * t56;
        let t58 = t29 * t34;
        let t60 = t57 * t58 + 1.0;
        let t61 = 1.0 / t60;
        let t66 = 1.0 + 2.0 / 9.0 * t28 * t31 * t49 * t61;
        let t70 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
    }
}
