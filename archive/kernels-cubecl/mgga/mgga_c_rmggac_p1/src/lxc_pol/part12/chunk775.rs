//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 775/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk775<F: Float>(t2084: F, t798: F, t7603: F, t7599: F, t25525: F, t27: F, t35917: F, t3851: F, t3826: F, t35884: F, t3814: F, t35871: F, t793: F) -> (F, F, F, F, F, F, F) {
    let t36114 = t2084 * t798;
    let t36115 = t7603 * t36114;
    let t36117 = t7599 * t36114;
    let t36119 = t25525 * t27;
    let t36127 = t3851 * t35917;
    let t36141 = t3826 * t35917;
    let t36152 = t3814 * t35884;
    let t36154 = t793 * t35871;
    (t36115, t36117, t36119, t36127, t36141, t36152, t36154)
}
