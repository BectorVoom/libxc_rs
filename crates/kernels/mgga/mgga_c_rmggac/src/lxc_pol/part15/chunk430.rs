//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 430/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk430<F: Float>(t504: F, t837: F, t4035: F, t529: F, t124: F, t235: F, t1679: F, t325: F) -> (F, F, F, F) {
    let t5019 = t504 * t837;
    let t5026 = t4035 * t529;
    let t5048 = t235 * t124;
    let t5055 = t1679 * t325;
    (t5019, t5026, t5048, t5055)
}
