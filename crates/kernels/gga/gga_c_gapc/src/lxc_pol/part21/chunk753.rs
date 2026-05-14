//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 753/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk753<F: Float>(t2405: F, t2982: F, t3391: F, t197: F, t7975: F, t1077: F, t2493: F, t3096: F, t3430: F, t154: F, t7073: F, t2580: F, t8686: F, t1026: F, t2497: F, t334: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9635 = t2982 * t2405;
    let t9636 = t3391 * t9635;
    let t9638 = t197 * t7975;
    let t9639 = t1077 * t9638;
    let t9641 = t3096 * t2493;
    let t9642 = t3430 * t9641;
    let t9644 = t7073 * t154;
    let t9645 = t8686 * t2580;
    let t9646 = t9644 * t9645;
    let t9648 = t2497 * t1026;
    let t9649 = t9648 * t334;
    (t9635, t9636, t9638, t9639, t9642, t9644, t9645, t9646, t9649)
}
