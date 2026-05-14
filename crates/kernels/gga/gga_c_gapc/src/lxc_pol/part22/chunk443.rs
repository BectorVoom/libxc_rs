//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 443/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk443<F: Float>(t2439: F, t2536: F, t869: F, t903: F, t291: F, t672: F, t332: F, t959: F) -> (F, F, F, F) {
    let t2537 = t2439 * t2536;
    let t2542 = t869 * t903;
    let t2545 = t672 * t291;
    let t2546 = t959 * t332;
    (t2537, t2542, t2545, t2546)
}
