//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 430/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk430<F: Float>(t2619: F, t310: F, t307: F, t2263: F, t2548: F, t140: F, t871: F, t6: F, t330: F) -> (F, F, F, F, F, F) {
    let t2620 = t310 * t2619;
    let t2622 = 0.60369177012421929547e-3 * t307 * t2620;
    let t2633 = t2548 * t2263;
    let t2638 = t871 * t140;
    let t2639 = t2638 * t6;
    let t2640 = t330 * t2639;
    (t2620, t2622, t2633, t2638, t2639, t2640)
}
