//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1046/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1046<F: Float>(t10093: F, t3351: F, t352: F, t7231: F, t875: F, t118: F, t2001: F, t38523: F, t570: F, t7720: F, t40001: F, t9222: F) -> (F, F, F) {
    let t47809 = t3351 * t7231 * t875 * t10093 * t352;
    let t47813 = t2001 * t118 * t38523 * t570;
    let t47814 = t7720 * t47813;
    let t47816 = t9222 * t40001;
    (t47809, t47814, t47816)
}
