//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1327/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1327<F: Float>(t1014: F, t29337: F, t29310: F, t28714: F, t28778: F, t28781: F, t20989: F, t303: F, t7931: F, t5870: F, t8175: F, t1458: F, t29400: F) -> (F, F, F, F, F, F, F) {
    let t102729 = t1014 * t29337;
    let t102731 = t1014 * t29310;
    let t102733 = t28714 * t28778;
    let t102735 = t28714 * t28781;
    let t102740 = t303 * t7931 * t20989;
    let t102743 = t303 * t5870 * t8175;
    let t102746 = t303 * t1458 * t29400;
    (t102729, t102731, t102733, t102735, t102740, t102743, t102746)
}
