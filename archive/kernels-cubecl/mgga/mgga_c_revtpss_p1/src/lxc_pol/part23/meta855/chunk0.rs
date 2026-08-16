//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2743/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743<F: Float>(t17729: F, t20922: F, t44425: F, t17396: F, t17617: F, t1222: F, t6658: F, t697: F, t6662: F, t12916: F, t20801: F, t5340: F) -> (F, F, F, F, F) {
    let t71908 = t17729 * t44425 * t20922;
    let t71920 = t17396 * t17617;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t71971 = t5340 * t12916 * t20801;
    (t71908, t71920, t71928, t71931, t71971)
}
