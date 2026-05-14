//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1078/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1078<F: Float>(t1979: F, t3604: F, t721: F, t2848: F, t2852: F, t5522: F, t5758: F, t7357: F, t7508: F, t9148: F, t9163: F, t261: F, t722: F, t9203: F, t5852: F, t7336: F, t9138: F, t9140: F, t9143: F, t9165: F, t9172: F, t9174: F) -> (F, F, F, F, F, F, F) {
    let t9451 = t3604 * t1979;
    let t9452 = t9451 * t721;
    let t9455 = t2852 * t2848;
    let t9462 = -t5758 + 0.12361111111111111111e-1 * t5522 + 0.24722222222222222223e-1 * t7357 - t7508 - 0.92708333333333333333e-2 * t9148 + 0.278125e-1 * t9163;
    let t9463 = t9462 * t261;
    let t9465 = t9203 * t722;
    let t9482 = 0.264729375e1 * t9138 - 0.3529725e1 * t9140 - 0.17648625e1 * t9143 + 0.3529725e1 * t9165 - t5852 + 0.68863333333333333333e0 * t5522 + 0.13772666666666666667e1 * t7357 - t7336 - 0.516475e0 * t9148 + 0.1549425e1 * t9163 - 0.157790625e0 * t9172 + 0.6311625e0 * t9174;
    (t9451, t9452, t9455, t9462, t9463, t9465, t9482)
}
