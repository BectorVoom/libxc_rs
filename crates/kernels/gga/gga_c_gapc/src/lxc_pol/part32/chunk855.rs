//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 855/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk855<F: Float>(t11569: F, t205: F, t3680: F, t5252: F, t3091: F, t3670: F, t19: F, t515: F, t147: F, t169: F, t125: F, t1482: F, t619: F, t11423: F, t3081: F, t11428: F, t144: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11570 = t11569 * t205;
    let t11572 = t5252 * t3680;
    let t11574 = t3670 * t3091;
    let t11576 = t515 * t19;
    let t11577 = t11576 * t147;
    let t11578 = t169 * t11577;
    let t11579 = t1482 * t125;
    let t11580 = t11579 * t619;
    let t11581 = t11578 * t11580;
    let t11584 = t169 * t11423 * t3081;
    let t11586 = t11428 * t144;
    (t11570, t11572, t11574, t11577, t11578, t11579, t11580, t11581, t11584, t11586)
}
