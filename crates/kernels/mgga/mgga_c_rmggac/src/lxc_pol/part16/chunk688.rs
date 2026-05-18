//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 688/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk688<F: Float>(t36: F, t9908: F, t854: F, t9876: F, t851: F, t9872: F, t793: F, t797: F, t3810: F, t9888: F, t3814: F, t3839: F, t9884: F) -> (F, F, F, F, F, F, F, F) {
    let t9909 = t9908 * t36;
    let t9911 = t854 * t9876;
    let t9913 = t851 * t9872;
    let t9915 = t793 * t9872;
    let t9917 = t797 * t9876;
    let t9919 = t3810 * t9888;
    let t9921 = t3814 * t9888;
    let t9923 = t3839 * t9884;
    (t9909, t9911, t9913, t9915, t9917, t9919, t9921, t9923)
}
