//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 761/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk761<F: Float>(t7524: F, t7525: F, t7527: F, t7529: F, t7531: F, t7535: F, t7538: F, t7541: F, t7544: F, t7547: F, t7550: F, t787: F, t780: F, t214: F, t7513: F, t2288: F, t531: F) -> (F, F, F, F, F, F) {
    let t7552 = -t7524 - 4.0 / 9.0 * t7525 + 2.0 / 9.0 * t7527 - 2.0 / 3.0 * t7529 + t7531 / 3.0 - 10.0 / 27.0 * t7535 + 4.0 / 3.0 * t7538 - 2.0 / 3.0 * t7541 - 2.0 * t7544 + 2.0 * t7547 - t7550 / 3.0;
    let t7553 = t787 * t7552;
    let t7555 = t780 * t7552;
    let t7557 = 1.0/pow_3_2(t214);
    let t7558 = t7557 * t7513;
    let t7560 = t531 * t2288;
    (t7552, t7553, t7555, t7557, t7558, t7560)
}
