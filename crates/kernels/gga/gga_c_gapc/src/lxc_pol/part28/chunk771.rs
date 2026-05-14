//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 771/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk771<F: Float>(t19: F, t761: F, t3114: F, t3440: F, t2200: F, t3439: F, t3438: F, t3103: F, t885: F, t3379: F, t2520: F, t2972: F, t3384: F, t787: F, t7927: F, t3396: F) -> (F, F, F, F, F, F) {
    let t9651 = t761 * t19;
    let t9652 = t9651 * t3114;
    let t9653 = t9652 * t3440;
    let t9655 = t2200 * t3439;
    let t9656 = t3438 * t9655;
    let t9658 = t885 * t3103;
    let t9659 = t9658 * t3379;
    let t9661 = t2520 * t2972;
    let t9662 = t9661 * t3384;
    let t9664 = t7927 * t787;
    let t9665 = t3396 * t9664;
    (t9652, t9653, t9656, t9659, t9662, t9665)
}
