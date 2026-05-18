//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 984/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk984<F: Float>(t238: F, t242: F, t9117: F, t3470: F, t940: F, t343: F, t9027: F, t9011: F, t6969: F, t6972: F, t7016: F, t9008: F, t9029: F) -> (F, F, F, F, F, F, F) {
    let t9119 = t238 * t242 * t9117;
    let t9121 = t940 * t3470;
    let t9123 = t238 * t242 * t9121;
    let t9125 = t343 * t9027;
    let t9127 = t238 * t242 * t9125;
    let t9134 = F::new(2.0) / F::new(3.0) * t9011;
    let t9135 = -t7016 + F::new(8.0) / F::new(9.0) * t6969 - t6972 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t9008 - t9134 + t9029;
    (t9119, t9121, t9123, t9125, t9127, t9134, t9135)
}
