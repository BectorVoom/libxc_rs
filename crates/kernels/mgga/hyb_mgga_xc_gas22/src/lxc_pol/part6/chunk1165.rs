//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1165/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1165<F: Float>(t238: F, t801: F, t9125: F, t1405: F, t6611: F, t2213: F, t3505: F, t3509: F, t260: F, t6993: F, t2599: F, t7109: F, t1409: F, t7075: F, t3513: F, t7150: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25348 = t238 * t801 * t9125;
    let t25359 = t238 * t6611 * t1405;
    let t25362 = t238 * t2213 * t3505;
    let t25365 = t238 * t2213 * t3509;
    let t25427 = t260 * t6993;
    let t25432 = t260 * t2599;
    let t25436 = t260 * t7109;
    let t25468 = t7075 * t1409;
    let t25520 = t3513 * t7150;
    (t25348, t25359, t25362, t25365, t25427, t25432, t25436, t25468, t25520)
}
