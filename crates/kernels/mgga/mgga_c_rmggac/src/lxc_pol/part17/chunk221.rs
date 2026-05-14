//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 221/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk221<F: Float>(t954: F, t957: F, t960: F, t964: F, t966: F, t969: F) -> (F,) {
    let t971 = -0.57538888888888888889e0 * t954 + 0.11507777777777777778e1 * t957 + 0.40256666666666666667e0 * t960 + 0.366775e-1 * t964 + 0.73355e-1 * t966 + 0.137975e0 * t969;
    (t971,)
}
