//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1458/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458<F: Float>(t11509: F, t6205: F, t2967: F, t6152: F, t3011: F, t6184: F, t2942: F, t2923: F, t6104: F, t3056: F, t6234: F, t378: F) -> (F, F, F, F, F, F, F) {
    let t64043 = t6205 * t11509;
    let t64060 = t6152 * t2967;
    let t64125 = t6184 * t3011;
    let t64319 = t6152 * t2942;
    let t64336 = t6104 * t2923;
    let t64686 = t6234 * t3056;
    let t64687 = t64686 * t378;
    (t64043, t64060, t64125, t64319, t64336, t64686, t64687)
}
