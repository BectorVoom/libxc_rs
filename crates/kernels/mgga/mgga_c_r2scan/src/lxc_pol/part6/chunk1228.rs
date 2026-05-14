//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1228/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1228<F: Float>(t2148: F, t22739: F, t7628: F, t1598: F, t524: F, t6291: F, t6296: F, t1541: F, t20: F, t525: F, t2294: F, t6106: F, t6108: F, t128: F, t20094: F, t6188: F) -> (F, F, F, F, F, F) {
    let t22741 = t7628 * t2148 * t22739;
    let t22744 = t524 * t1598 * t6291;
    let t22745 = t22744 * t6296;
    let t22749 = t524 * t525 * t1541 * t20;
    let t22756 = t6106 * t2294 * t6108;
    let t22766 = t20094 * t128;
    let t22767 = t6188 * t22766;
    (t22741, t22744, t22745, t22749, t22756, t22767)
}
