//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1228/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1228<F: Float>(t2670: F, t7467: F, t2644: F, t2668: F, t115: F, t2341: F, t911: F, t2718: F, t297: F, t7835: F, t770: F, t2811: F, t7420: F) -> (F, F, F, F, F) {
    let t25355 = t7467 * t2670;
    let t25357 = t2668 * t25355 * t2644;
    let t25360 = t2341 * t911 * t115;
    let t25361 = t2718 * t25360;
    let t25364 = t7835 * t297;
    let t25365 = t25364 * t770;
    let t25369 = t2811 * t7420;
    (t25355, t25357, t25361, t25365, t25369)
}
