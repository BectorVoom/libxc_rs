//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1157/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1157<F: Float>(t1056: F, t3305: F, t2213: F, t238: F, t3344: F, t3348: F, t1342: F, t6611: F, t801: F, t8693: F, t8697: F, t8701: F, t2187: F, t3313: F, t1333: F, t6578: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24587 = 8.0 * t3305 * t1056;
    let t24658 = t238 * t2213 * t3344;
    let t24661 = t238 * t2213 * t3348;
    let t24664 = t238 * t6611 * t1342;
    let t24667 = t238 * t801 * t8693;
    let t24670 = t238 * t801 * t8697;
    let t24673 = t238 * t801 * t8701;
    let t24699 = t3313 * t2187;
    let t24702 = t1333 * t6578;
    (t24587, t24658, t24661, t24664, t24667, t24670, t24673, t24699, t24702)
}
