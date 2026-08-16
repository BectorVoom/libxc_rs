//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 511/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk511(t209: f64, t221: f64, t5590: f64, t1212: f64, t589: f64, t605: f64, t1494: f64) -> (f64, f64, f64, f64, f64) {
    let t5592 = t221 * t5590 * t209;
    let t5595 = t589 * t1212;
    let t5597 = t221 * t5595 * t209;
    let t5600 = t605 * t1212;
    let t5601 = t5600 * t209;
    let t5602 = t221 * t5601;
    let t5605 = t1494 * t209;
    (t5592, t5597, t5601, t5602, t5605)
}
