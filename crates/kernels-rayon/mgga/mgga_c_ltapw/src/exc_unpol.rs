//! MGGA_C_LTAPW exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ltapw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ltapw_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_ltafrac: f64,
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
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t11 = pow_1_3(rho[ip]);
        let t12 = t11 * t11;
        let t15 = M_CBRT6;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t25 = rmath::pow(5.0 / 9.0 * tau[ip] * t9 / t12 / rho[ip] * t15 / t19, 3.0 / 5.0 * param_ltafrac);
        let t26 = rho[ip] * t25;
        let t27 = pow_1_3(t26);
        let t30 = t5 * t7 / t27;
        let t32 = 1.0 + 0.053425 * t30;
        let t33 = rmath::sqrt(t30);
        let t36 = pow_3_2(t30);
        let t38 = t2 * t2;
        let t39 = t4 * t4;
        let t40 = t38 * t39;
        let t41 = t27 * t27;
        let t44 = t40 * t6 / t41;
        let t46 = 3.79785 * t33 + 0.8969 * t30 + 0.204775 * t36 + 0.123235 * t44;
        let t49 = 1.0 + 16.081824322151103 / t46;
        let t50 = rmath::ln(t49);
        let t52 = 0.062182 * t32 * t50;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t62 = (2.0 * t56 - 2.0) / (2.0 * t8 - 2.0);
        let t64 = 1.0 + 0.0278125 * t30;
        let t69 = 5.1785 * t33 + 0.905775 * t30 + 0.1100325 * t36 + 0.1241775 * t44;
        let t72 = 1.0 + 29.608574643216677 / t69;
        let t73 = rmath::ln(t72);
        let t76 = 0.019751789702565206 * t62 * t64 * t73;
        let tzk0 = -t52 + t76;
        zk[ip] += tzk0;
    }
}
