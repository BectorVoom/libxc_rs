//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 620/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk620<F: Float>(t3278: F, t4391: F, t3952: F, t1567: F, t1308: F, sigma0: F) -> (F, F, F, F) {
    let t4392 = t4391 * t3278;
    let t4393 = t3952 * t4392;
    let t4396 = t1567 * sigma0;
    let t4397 = t4396 * t1308;
    (t4392, t4393, t4396, t4397)
}
