//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 950/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk950<F: Float>(t238: F, t242: F, t9125: F, t9011: F, t6969: F, t6972: F, t7016: F, t9008: F, t9029: F, t950: F, t957: F, t2490: F, t3485: F, t2496: F, t3490: F, t952: F) -> (F, F, F, F, F, F, F) {
    let t9127 = t238 * t242 * t9125;
    let t9134 = 2.0 / 3.0 * t9011;
    let t9135 = -t7016 + 8.0 / 9.0 * t6969 - t6972 / 3.0 + 4.0 / 9.0 * t9008 - t9134 + t9029;
    let t9136 = t950 * t9135;
    let t9138 = t957 * t9135;
    let t9140 = t3485 * t2490;
    let t9142 = t2496 * t3490;
    let t9143 = t9142 * t952;
    (t9127, t9134, t9135, t9136, t9138, t9140, t9143)
}
