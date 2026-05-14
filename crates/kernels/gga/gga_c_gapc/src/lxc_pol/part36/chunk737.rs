//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 737/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk737<F: Float>(t2580: F, t8686: F, t9644: F, t1026: F, t2497: F, t334: F, t19: F, t761: F, t3114: F, t3440: F, t2200: F, t3439: F, t3438: F, t3103: F, t885: F, t3379: F) -> (F, F, F, F, F, F, F) {
    let t9645 = t8686 * t2580;
    let t9646 = t9644 * t9645;
    let t9648 = t2497 * t1026;
    let t9649 = t9648 * t334;
    let t9651 = t761 * t19;
    let t9652 = t9651 * t3114;
    let t9653 = t9652 * t3440;
    let t9655 = t2200 * t3439;
    let t9656 = t3438 * t9655;
    let t9658 = t885 * t3103;
    let t9659 = t9658 * t3379;
    (t9645, t9646, t9649, t9652, t9653, t9656, t9659)
}
