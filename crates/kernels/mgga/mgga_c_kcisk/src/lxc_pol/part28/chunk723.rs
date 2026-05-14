//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 723/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk723<F: Float>(t8865: F, t8963: F, t752: F, t2594: F, t7293: F) -> (F, F, F, F) {
    let t8964 = t8865 + t8963;
    let t8965 = t8964 * t752;
    let t8967 = 2.0 * t7293 * t2594;
    let t8968 = t2594 * t2594;
    (t8964, t8965, t8967, t8968)
}
