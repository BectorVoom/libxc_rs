//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 255/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk255<F: Float>(t1163: F, t1224: F, t1225: F, t1223: F, t357: F) -> (F, F, F, F) {
    let t1227 = t1224 * t1225 * t1163;
    let t1229 = -t1223 - 0.17808333333333333333e-1 * t1227;
    let t1232 = t357 * t357;
    let t1233 = 1.0 / t1232;
    (t1227, t1229, t1232, t1233)
}
