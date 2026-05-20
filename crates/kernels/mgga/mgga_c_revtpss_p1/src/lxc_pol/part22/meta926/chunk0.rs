//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3149/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149<F: Float>(t17351: F, t17354: F, t56756: F, t3588: F, t3611: F, t12904: F, t5293: F, t12959: F, t17569: F, t11262: F, t1261: F, t5269: F) -> (F, F, F, F, F) {
    let t56758 = t17351 * t56756 * t17354;
    let t56760 = t3611 * t3588;
    let t56785 = t5293 * t12904;
    let t56787 = t17569 * t12959;
    let t56790 = t1261 * t11262 * t5269;
    (t56758, t56760, t56785, t56787, t56790)
}
