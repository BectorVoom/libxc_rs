//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 482/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk482<F: Float>(t1084: F, t1737: F, t2546: F, t6: F, t2597: F, t186: F, t277: F) -> (F, F, F) {
    let t2655 = t1084 * t1737;
    let t2656 = t2546 * t6;
    let t2657 = t2597 * t2656;
    let t2660 = t277 * t186;
    (t2655, t2657, t2660)
}
