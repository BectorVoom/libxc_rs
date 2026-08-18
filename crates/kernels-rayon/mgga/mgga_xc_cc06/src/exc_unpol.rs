//! MGGA_XC_CC06 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_cc06_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t9 = pow_1_3(zeta_threshold);
        let t11 = piecewise3(1.0 <= zeta_threshold, t9 * zeta_threshold, 1.0);
        let t12 = pow_1_3(rho[ip]);
        let t16 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t11 * t12);
        let t18 = 1.0 / M_PI;
        let t19 = pow_1_3(t18);
        let t20 = t4 * t19;
        let t21 = M_CBRT4;
        let t22 = t21 * t21;
        let t25 = t20 * t22 / t12;
        let t27 = 1.0 + 0.053425 * t25;
        let t28 = f64::sqrt(t25);
        let t31 = pow_3_2(t25);
        let t33 = t4 * t4;
        let t34 = t19 * t19;
        let t35 = t33 * t34;
        let t36 = t12 * t12;
        let t37 = 1.0 / t36;
        let t39 = t35 * t21 * t37;
        let t41 = 3.79785 * t28 + 0.8969 * t25 + 0.204775 * t31 + 0.123235 * t39;
        let t44 = 1.0 + 16.081824322151103 / t41;
        let t45 = f64::ln(t44);
        let t50 = M_CBRT2;
        let t54 = (2.0 * t11 - 2.0) / (2.0 * t50 - 2.0);
        let t56 = 1.0 + 0.0278125 * t25;
        let t61 = 5.1785 * t28 + 0.905775 * t25 + 0.1100325 * t31 + 0.1241775 * t39;
        let t64 = 1.0 + 29.608574643216677 / t61;
        let t65 = f64::ln(t64);
        let t69 = 2.0 * t16 - 0.062182 * t27 * t45 + 0.019751789702565206 * t54 * t56 * t65;
        let t70 = t33 * t21;
        let t71 = t34 * lapl[ip];
        let t73 = 1.0 / t36 / rho[ip];
        let t75 = t70 * t71 * t73;
        let t77 = -0.0007 + 0.002 * t75;
        let t79 = 1.0 + 0.0065 * t75;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let tzk0 = t69 * t82;
        zk[ip] += tzk0;
    }
}
