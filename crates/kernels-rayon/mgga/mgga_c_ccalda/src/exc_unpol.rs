//! MGGA_C_CCALDA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ccalda_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = 1.0 + param_c;
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / rho[ip];
        let t8 = rho[ip] * rho[ip];
        let t10 = 1.0 / t4 / t8;
        let t13 = tau[ip] * t6 - sigma[ip] * t10 / 8.0;
        let t14 = t2 * t13;
        let t15 = M_CBRT6;
        let t16 = t14 * t15;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t26 = t15 * t20 * t22;
        let t29 = 1.0 + 5.0 / 9.0 * param_c * t13 * t26;
        let t30 = 1.0 / t29;
        let t31 = M_CBRT3;
        let t32 = 1.0 / M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t39 = t34 * t36 / t3;
        let t41 = 1.0 + 0.053425 * t39;
        let t42 = rmath::sqrt(t39);
        let t45 = pow_3_2(t39);
        let t47 = t31 * t31;
        let t48 = t33 * t33;
        let t49 = t47 * t48;
        let t52 = t49 * t35 / t4;
        let t54 = 3.79785 * t42 + 0.8969 * t39 + 0.204775 * t45 + 0.123235 * t52;
        let t57 = 1.0 + 16.081979498692537 / t54;
        let t58 = rmath::ln(t57);
        let t62 = pow_1_3(zeta_threshold);
        let t64 = piecewise3(1.0 <= zeta_threshold, t62 * zeta_threshold, 1.0);
        let t70 = (2.0 * t64 - 2.0) / (2.0 * t21 - 2.0);
        let t72 = 1.0 + 0.0278125 * t39;
        let t77 = 5.1785 * t42 + 0.905775 * t39 + 0.1100325 * t45 + 0.1241775 * t52;
        let t80 = 1.0 + 29.608749977793437 / t77;
        let t81 = rmath::ln(t80);
        let t85 = -0.0621814 * t41 * t58 + 0.0197516734986138 * t70 * t72 * t81;
        let t87 = t23 * t30 * t85;
        let t89 = 5.0 / 9.0 * t16 * t87;
        let t90 = t23 * t30;
        let t93 = 1.0 - 5.0 / 9.0 * t16 * t90;
        let t94 = t93 * t85;
        let tzk0 = t89 + t94;
        zk[ip] += tzk0;
    }
}
