//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1276/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1276<F: Float>(t10948: F, t238: F, t801: F, t242: F, t2466: F, t4234: F, t10911: F, t940: F, t29786: F, t343: F, t2213: F, t4261: F, t3470: F, t25342: F, t25345: F, t25348: F, t25359: F, t25362: F, t25365: F) -> (F, F, F, F, F, F, F) {
    let t29884 = t238 * t801 * t10948;
    let t29888 = t238 * t242 * t2466 * t4234;
    let t29892 = t238 * t242 * t940 * t10911;
    let t29896 = t238 * t242 * t343 * t29786;
    let t29905 = t238 * t2213 * t4261;
    let t29907 = t3470 * t3470;
    let t29909 = t238 * t242 * t29907;
    let t29911 = -0.33114e0 * t29884 + 0.248355e0 * t29888 + 0.49671e0 * t29892 + 0.248355e0 * t29896 - 0.33114e0 * t25342 - 0.66228e0 * t25345 - 0.33114e0 * t25348 - 0.14717333333333333333e1 * t25359 + 0.11038e1 * t25362 + 0.11038e1 * t25365 + 0.27595e0 * t29905 + 0.49671e0 * t29909;
    (t29884, t29888, t29892, t29896, t29905, t29909, t29911)
}
