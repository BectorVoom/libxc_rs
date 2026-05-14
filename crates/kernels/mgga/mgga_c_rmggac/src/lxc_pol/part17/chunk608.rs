//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 608/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk608<F: Float>(t7934: F, t9081: F, t7933: F, t1392: F, t202: F, t461: F, t674: F) -> (F, F, F, F, F) {
    let t9082 = t7934 * t9081;
    let t9083 = t7933 * t9082;
    let t9085 = t1392 * t202;
    let t9086 = t9085 * t461;
    let t9087 = t9086 * t674;
    (t9082, t9083, t9085, t9086, t9087)
}
