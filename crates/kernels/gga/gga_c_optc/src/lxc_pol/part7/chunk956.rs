//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 956/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk956<F: Float>(t22647: F, t1864: F, t1867: F, t22075: F, t601: F, t22403: F, t22406: F, t22410: F, t22632: F, t22634: F, t22636: F, t22638: F, t22641: F, t22644: F, t22646: F, t592: F, t6326: F) -> (F, F, F, F) {
    let t22648 = 192.0 * t22647;
    let t22652 = 0.51947267698127589897e2 * t601 * t1864 * t22075 * t1867;
    let t22653 = t22632 - t22634 - t22403 + t22636 + t22638 - t22641 - t22644 - t22406 - t22410 + t22646 - t22648 - t22652;
    let t22655 = 480.0 * t6326 * t592;
    (t22648, t22652, t22653, t22655)
}
