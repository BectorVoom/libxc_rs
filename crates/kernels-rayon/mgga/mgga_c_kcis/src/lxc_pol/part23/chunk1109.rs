//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1109/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1109(t28610: f64, t7953: f64, t28583: f64, t28585: f64, t28587: f64, t28590: f64, t28592: f64, t28595: f64, t28598: f64, t28600: f64, t28602: f64, t28604: f64, t28606: f64, t28608: f64) -> (f64, f64) {
    let t28611 = t28610 * t7953;
    let t28613 = -t28583 / 24.0_f64 + t28585 / 128.0_f64 + t28587 / 18.0_f64 - t28590 / 16.0_f64 - t28592 / 128.0_f64 + t28595 / 6.0_f64 - t28598 / 16.0_f64 + t28600 / 128.0_f64 + t28602 / 8.0_f64 - t28604 / 96.0_f64 - t28606 / 24.0_f64 - t28608 / 96.0_f64 - t28611 / 9.0_f64;
    (t28611, t28613)
}
