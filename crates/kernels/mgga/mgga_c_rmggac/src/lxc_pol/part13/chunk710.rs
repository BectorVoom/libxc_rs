//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 710/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk710<F: Float>(t25525: F, t27: F, t35917: F, t3851: F, t3826: F, t35884: F, t3814: F, t35871: F, t793: F, t344: F, t3899: F, t265: F, t5245: F, t35863: F, t797: F, t35875: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36119 = t25525 * t27;
    let t36127 = t3851 * t35917;
    let t36141 = t3826 * t35917;
    let t36152 = t3814 * t35884;
    let t36154 = t793 * t35871;
    let t36156 = t344 * t3899;
    let t36158 = t5245 * t265;
    let t36160 = t797 * t35863;
    let t36166 = t793 * t35875;
    (t36119, t36127, t36141, t36152, t36154, t36156, t36158, t36160, t36166)
}
