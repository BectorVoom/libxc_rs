//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 687/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk687<F: Float>(t35238: F, t2019: F, t2164: F, t7352: F, t7764: F, t7556: F, t7553: F, t7555: F, t288: F, t49: F, t2038: F, t7756: F, t7933: F, t2604: F, t7779: F, t674: F, t7433: F, t7715: F) -> (F, F, F, F, F, F, F) {
    let t35239 = 0.45731474687362542471e-3 * t35238;
    let t35242 = t2019 * t7764 * t2164 * t7352;
    let t35244 = t2164 * t7556;
    let t35246 = t7553 * t7555 * t35244;
    let t35253 = t49 * t288;
    let t35256 = t7933 * t2038 * t35253 * t7756;
    let t35262 = t2604 * t7779;
    let t35265 = t7433 * t7715 * t674;
    (t35239, t35242, t35246, t35253, t35256, t35262, t35265)
}
