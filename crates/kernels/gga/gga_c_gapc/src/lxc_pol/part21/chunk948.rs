//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 948/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk948<F: Float>(t11579: F, t619: F, t11578: F, t11423: F, t169: F, t3081: F, t11428: F, t144: F, t1461: F, t1030: F, t1908: F, t203: F) -> (F, F, F, F, F, F) {
    let t11580 = t11579 * t619;
    let t11581 = t11578 * t11580;
    let t11584 = t169 * t11423 * t3081;
    let t11586 = t11428 * t144;
    let t11587 = t1461 * t11586;
    let t11588 = t1030 * t11587;
    let t11589 = t1908 * t203;
    (t11580, t11581, t11584, t11587, t11588, t11589)
}
