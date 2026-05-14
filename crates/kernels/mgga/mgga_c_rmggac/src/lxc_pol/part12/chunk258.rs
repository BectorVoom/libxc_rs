//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 258/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk258<F: Float>(t1163: F, t1166: F, t1168: F, t1174: F, t1175: F, t1240: F, t228: F, t458: F, t462: F) -> (F,) {
    let t1243 = t1163 * t228 + t1166 * t228 + t458 * t1168 / 2.0 - 5.0 / 16.0 * t1174 * t1175 + t462 * t1240 / 4.0;
    (t1243,)
}
