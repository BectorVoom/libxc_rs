//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 748/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk748<F: Float>(t8665: F, t8722: F, t8773: F, t8827: F, t8875: F, t8924: F, t8971: F, t9005: F, t9046: F, t9095: F, t9134: F, t9170: F, t9212: F, t9265: F, t9322: F, t9366: F) -> (F,) {
    let t9370 = t8665 + t8722 + t8773 + t8827 + t8875 + t8924 + t8971 + t9005 + t9046 + t9095 + t9134 + t9170 + t9212 + t9265 + t9322 + t9366;
    (t9370,)
}
