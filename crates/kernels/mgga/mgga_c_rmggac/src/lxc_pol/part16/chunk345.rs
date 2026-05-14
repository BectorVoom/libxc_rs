//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 345/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk345<F: Float>(t2085: F, t648: F, t2069: F, t793: F, t2074: F, t797: F, t265: F, t305: F, t22: F) -> (F, F, F, F, F) {
    let t2086 = t648 * t2085;
    let t2094 = t793 * t2069;
    let t2096 = t797 * t2074;
    let t2098 = t305 * t265;
    let t2100 = t797 * t22;
    (t2086, t2094, t2096, t2098, t2100)
}
