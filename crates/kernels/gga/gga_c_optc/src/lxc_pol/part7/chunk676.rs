//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 676/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk676<F: Float>(t56: F, t6567: F, t202: F, t188: F, t2204: F, t737: F, t3649: F, t3696: F, t2211: F, t723: F, t2217: F, t720: F, t722: F, t179: F, t2219: F, t727: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6568 = t6567 * t56;
    let t6569 = t6568 * t202;
    let t6571 = 455.0 / 27.0 * t188 * t6569;
    let t6572 = t737 * t2204;
    let t6576 = -0.60319259259259259259e1 * t3649 - 0.54733333333333333333e-2 * t3696;
    let t6578 = t2211 * t723;
    let t6581 = t720 * t2217;
    let t6586 = t722 * t722;
    let t6587 = 1.0 / t6586;
    let t6588 = t179 * t6587;
    let t6589 = t2219 * t727;
    (t6568, t6569, t6571, t6572, t6576, t6578, t6581, t6586, t6587, t6588, t6589)
}
