//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1073/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1073(t137163: f64, t137172: f64, t137180: f64, t145038: f64, t145042: f64, t145045: f64, t145048: f64, t145051: f64, t145055: f64, t145058: f64, t145061: f64, t145588: f64, t145590: f64, t145595: f64, t145598: f64, t145601: f64) -> f64 {
    let t145875 = t145038 / 3.0_f64 + t145042 / 12.0_f64 - t145045 / 3.0_f64 - t145048 - 2.0_f64 / 9.0_f64 * t145051 + 2.0_f64 / 27.0_f64 * t137163 + 4.0_f64 / 9.0_f64 * t145055 - 4.0_f64 / 27.0_f64 * t145058 + t145061 / 18.0_f64 - t145588 / 3.0_f64 + t145590 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t137172 - 8.0_f64 / 9.0_f64 * t137180 + 2.0_f64 / 3.0_f64 * t145595 - 4.0_f64 / 9.0_f64 * t145598 + 4.0_f64 / 9.0_f64 * t145601;
    t145875
}
