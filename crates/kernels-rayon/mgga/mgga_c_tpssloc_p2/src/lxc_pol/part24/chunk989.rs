//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 989/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk989(t11136: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t11165: f64, t11170: f64, t11174: f64, t449: f64) -> f64 {
    let t11176 = -t11136 + 0.12361111111111111111e-1_f64 * t11137 + 0.61805555555555555556e-2_f64 * t11139 - 0.18541666666666666667e-1_f64 * t11141 - 0.92708333333333333334e-2_f64 * t11143 + 0.10300925925925925926e-1_f64 * t11150 - 0.37083333333333333333e-1_f64 * t11156 - 0.18541666666666666666e-1_f64 * t11161 + 0.55625000000000000001e-1_f64 * t11165 + 0.55625000000000000001e-1_f64 * t11170 + 0.92708333333333333333e-2_f64 * t11174;
    let t11177 = t11176 * t449;
    t11177
}
