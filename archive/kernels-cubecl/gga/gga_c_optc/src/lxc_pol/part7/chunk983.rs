//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 983/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk983<F: Float>(t2269: F, t2434: F, t2748: F, t883: F, t2667: F, t2769: F, t852: F, t3883: F, t1659: F, t141: F, t872: F, t2811: F) -> (F, F, F, F, F, F, F, F) {
    let t11374 = t2434 * t2269;
    let t11398 = t2748 * t883;
    let t11399 = t11398 * t2667;
    let t11450 = t2769 * t852;
    let t11451 = t11450 * t3883;
    let t11454 = t1659 * t852;
    let t11455 = t11454 * t3883;
    let t11472 = t872 * t141;
    let t11473 = t2811 * t11472;
    (t11374, t11398, t11399, t11450, t11451, t11454, t11455, t11473)
}
