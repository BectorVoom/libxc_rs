//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3010/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3010<F: Float>(t10811: F, t14678: F, t10871: F, t1558: F, t10726: F, t10943: F, t2661: F, t4352: F, t14547: F, t40693: F, t14917: F, t2475: F, t2662: F) -> (F, F, F, F, F) {
    let t50472 = t10811 * t14678;
    let t50474 = t1558 * t10871;
    let t50493 = t2661 * t10726 * t4352 * t10943;
    let t50497 = t2661 * t40693 * t4352 * t14547;
    let t50502 = t2661 * t2662 * t2475 * t1558 * t14917;
    (t50472, t50474, t50493, t50497, t50502)
}
