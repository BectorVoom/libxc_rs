//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1087/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1087<F: Float>(t2206: F, t4121: F, t791: F, t10577: F, t796: F, t238: F, t4131: F, t801: F, t1329: F, t3309: F, t242: F, t4135: F) -> (F, F, F, F, F, F, F) {
    let t10590 = t2206 * t4121;
    let t10591 = t10590 * t791;
    let t10593 = t796 * t10577;
    let t10598 = t238 * t801 * t4131;
    let t10600 = t1329 * t3309;
    let t10602 = t238 * t242 * t10600;
    let t10605 = t238 * t801 * t4135;
    (t10590, t10591, t10593, t10598, t10600, t10602, t10605)
}
