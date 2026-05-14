//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 672/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk672<F: Float>(t43: F, t1891: F, t607: F, t6533: F, t6534: F, t6537: F, t6541: F, t1026: F, t52: F, t1897: F, t553: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t6545 = piecewise3(t44, 0.0, 8.0 / 27.0 * t6533 * t6534 - 2.0 / 3.0 * t6537 * t1891 + 2.0 / 3.0 * t607 * t6541);
    let t6547 = 1.0 / t52 / t1026;
    let t6548 = t1897 * t553;
    (t6545, t6547, t6548)
}
