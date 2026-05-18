//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 832/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk832<F: Float>(t3851: F, t40897: F, t3839: F, t40905: F, t25640: F, t36: F, t3826: F, t25518: F, t41027: F, t793: F, t41035: F, t797: F) -> (F, F, F, F, F, F, F) {
    let t41146 = t3851 * t40897;
    let t41160 = t3839 * t40905;
    let t41165 = t25640 * t36;
    let t41170 = t3826 * t40897;
    let t41176 = t25518 * t36;
    let t41191 = t793 * t41027;
    let t41195 = t797 * t41035;
    (t41146, t41160, t41165, t41170, t41176, t41191, t41195)
}
