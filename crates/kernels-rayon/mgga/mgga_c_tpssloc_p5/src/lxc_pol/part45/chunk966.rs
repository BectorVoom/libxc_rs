//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 966/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk966(t31744: f64, t4034: f64, t652: f64, t6534: f64, t7156: f64, t12823: f64, t8533: f64, t31772: f64, t112521: f64, t112523: f64, t114541: f64, t114543: f64, t114554: f64, t114559: f64, t12734: f64, t22461: f64, t23918: f64, t23929: f64, t26103: f64, t31726: f64, t6517: f64, t6862: f64, t7056: f64, t7057: f64, t8529: f64) -> f64 {
    let t114561 = 4.0_f64 * t4034 * t31744;
    let t114564 = 4.0_f64 * t652 * t7156 * t6534;
    let t114566 = 2.0_f64 * t12823 * t8533;
    let t114568 = 4.0_f64 * t4034 * t31772;
    let t114569 = -4.0_f64 * t652 * t6862 * t7056 - 4.0_f64 * t12734 * t8529 - 2.0_f64 * t12823 * t8529 - 4.0_f64 * t22461 * t7057 - 2.0_f64 * t23918 * t6517 - 4.0_f64 * t23929 * t6517 - 4.0_f64 * t26103 * t7057 - 4.0_f64 * t31726 * t4034 - t112521 - t112523 - t114541 - t114543 - t114554 - t114559 - t114561 - t114564 - t114566 - t114568;
    t114569
}
