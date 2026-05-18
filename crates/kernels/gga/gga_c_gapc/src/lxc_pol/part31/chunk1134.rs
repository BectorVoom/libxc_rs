//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1134/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1134<F: Float>(t126: F, t932: F, t1038: F, t11925: F, t16826: F, t19: F, t7877: F, t15615: F, t3327: F, t17713: F, t23466: F, t8676: F) -> (F, F, F, F, F) {
    let t30153 = t932 * t126;
    let t30158 = t11925 * t1038 * t7877 * t19 * t16826;
    let t30167 = t3327 * t15615;
    let t30187 = t3327 * t17713;
    let t30288 = t8676 * t23466;
    (t30153, t30158, t30167, t30187, t30288)
}
