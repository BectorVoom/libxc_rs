//! MGGA_C_CC exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cc_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(1.0 <= zeta_threshold, t35 * zeta_threshold, 1.0);
        let t40 = M_CBRT2;
        let t44 = (2.0 * t37 - 2.0) / (2.0 * t40 - 2.0);
        let t46 = 1.0 + 0.0278125 * t11;
        let t51 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let tzk0 = -t33 + t58;
        zk[ip] += tzk0;
    }
}
