//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 453/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk453<F: Float>(t4118: F, t4056: F, t4062: F, t4064: F, t4074: F, t4077: F, t4080: F, t4083: F, t4089: F, t4101: F, t4106: F, t4111: F, t5375: F, t5376: F, t5971: F, t5977: F, t5978: F) -> (F, F) {
    let t5981 = 12.0 * t4118;
    let t5982 = -t4056 + t4062 + t4064 + t5375 - t5376 - t4074 - t5971 - t4077 - t4080 + t4083 + t5977 + t4089 - t4101 + t4106 + t4111 + t5978;
    (t5981, t5982)
}
