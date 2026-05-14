//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 696/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk696<F: Float>(t3796: F, t8171: F, t3482: F, t2059: F, t2152: F, t3485: F) -> (F, F, F) {
    let t8172 = t3796 * t8171;
    let t8173 = t3482 * t8172;
    let t8175 = t2059 * t2152;
    let t8176 = t3485 * t8175;
    (t8172, t8173, t8176)
}
