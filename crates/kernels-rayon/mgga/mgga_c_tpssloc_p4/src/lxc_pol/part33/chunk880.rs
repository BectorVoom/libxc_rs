//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 880/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk880(t5689: f64, t892: f64, t3216: f64, t5946: f64, t5717: f64, t699: f64, t5720: f64, t5723: f64, t5769: f64, t942: f64, t5737: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17195 = t5689 * t892;
    let t17202 = t5946 * t3216;
    let t17286 = t699 * t5717;
    let t17288 = t699 * t5720;
    let t17290 = t699 * t5723;
    let t17355 = t5769 * t942;
    let t17428 = t5737 * t923;
    (t17195, t17202, t17286, t17288, t17290, t17355, t17428)
}
