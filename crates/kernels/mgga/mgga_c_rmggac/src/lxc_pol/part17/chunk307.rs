//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 307/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk307<F: Float>(t1011: F, t1014: F, t1017: F, t1019: F, t1050: F, t1087: F, t1094: F, t1104: F, t1112: F, t1140: F, t948: F, t982: F) -> F {
    let t1846 = -t1019 - t1094 + t1011 + t1014 + t1017 - t1050 + t1112 + t1104 + t948 + t982 - t1087 + t1140;
    t1846
}
