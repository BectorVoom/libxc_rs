//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1327/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1327(t1014: f64, t29337: f64, t29310: f64, t28714: f64, t28778: f64, t28781: f64, t20989: f64, t303: f64, t7931: f64, t5870: f64, t8175: f64, t1458: f64, t29400: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102729 = t1014 * t29337;
    let t102731 = t1014 * t29310;
    let t102733 = t28714 * t28778;
    let t102735 = t28714 * t28781;
    let t102740 = t303 * t7931 * t20989;
    let t102743 = t303 * t5870 * t8175;
    let t102746 = t303 * t1458 * t29400;
    (t102729, t102731, t102733, t102735, t102740, t102743, t102746)
}
