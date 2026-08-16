//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 632/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk632(t1055: f64, t5943: f64, t1052: f64, t1635: f64, t388: f64, t4557: f64, t4660: f64, t5849: f64, t5851: f64, t5915: f64, t5920: f64, t1637: f64) -> (f64, f64, f64) {
    let t5944 = t1055 * t5943;
    let t5946 = 2.0_f64 * t1052 * t5920 - t1052 * t5944 - 2.0_f64 * t1635 * t4557 - 2.0_f64 * t1635 * t4660 + t388 * t5849 + 2.0_f64 * t388 * t5851 + t388 * t5915;
    let t5950 = t1637 * t1637;
    (t5944, t5946, t5950)
}
