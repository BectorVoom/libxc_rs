//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 977/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk977<F: Float>(t17336: F, t662: F, t16022: F, t10798: F, t7238: F, t5013: F, t4893: F) -> (F, F, F, F) {
    let t17345 = t17336 * t662;
    let t17346 = t17345 * t16022;
    let t17349 = t10798 * t7238;
    let t17351 = 0.11993859144118211475e-1 * t5013 * t17349;
    let t17353 = t4893 * t662;
    (t17345, t17346, t17351, t17353)
}
