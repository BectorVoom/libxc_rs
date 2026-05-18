//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 532/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk532<F: Float>(t3073: F, t3076: F, t644: F, t998: F, t169: F, t442: F, t599: F) -> (F, F, F, F) {
    let t3077 = t3073 * t3076;
    let t3079 = t998 * t644;
    let t3080 = t169 * t3079;
    let t3081 = t442 * t599;
    (t3077, t3079, t3080, t3081)
}
