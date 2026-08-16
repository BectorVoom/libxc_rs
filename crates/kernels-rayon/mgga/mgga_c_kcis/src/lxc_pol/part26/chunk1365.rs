//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1365/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1365(t1307: f64, t20916: f64, t5709: f64, t102237: f64, t102240: f64, t102245: f64, t102262: f64, t102292: f64, t102299: f64, t102303: f64, t103340: f64, t27369: f64, t27459: f64, t28369: f64, t28439: f64, t29289: f64, t29314: f64, t7908: f64) -> (f64, f64) {
    let t103423 = t5709 * t20916 * t1307;
    let t103438 = -0.16581944444444444444e-2_f64 * t102237 - 0.88437037037037037033e-2_f64 * t102240 - 0.33163888888888888888e-2_f64 * t102245 + 0.30918233506944444444e-4_f64 * t27369 * t103423 + 0.46336805555555555557e-3_f64 * t28369 * t28439 + 0.46336805555555555556e-3_f64 * t27459 * t29314 + 0.11054629629629629629e-2_f64 * t102262 - 0.13901041666666666667e-2_f64 * t27459 * t29289 - 0.13901041666666666667e-2_f64 * t7908 * t103340 + 0.66327777777777777776e-2_f64 * t102292 - 0.1492375e-1_f64 * t102299 + 0.33163888888888888888e-2_f64 * t102303;
    (t103423, t103438)
}
