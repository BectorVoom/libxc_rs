//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1265/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1265<F: Float>(t11107: F, t11116: F, t2538: F, t2539: F, t2554: F, t2560: F, t29644: F, t29648: F, t29650: F, t29652: F, t29654: F, t29656: F, t29658: F, t29660: F, t29663: F, t29666: F, t29669: F, t29671: F, t29674: F, t29684: F, t3550: F, t4297: F, t7002: F, t7059: F, t9241: F) -> (F,) {
    let t29694 = t29644 - t29648 - t29650 - t29652 + t29654 + t29656 - t29658 - t29660 - t29663 - t29666 + t29669 + t29671 - t29674 - 2.0 * t2538 * t4297 * t2554 - 0.19298375398431042081e3 * t7002 * t11107 * t2539 + 0.32163958997385070134e2 * t2560 * t11107 * t2554 + 0.2069040516770936012e4 * t7059 * t29684 * t2539 + 0.64327917994770140268e2 * t2560 * t3550 * t9241 + 0.2069040516770936012e4 * t7059 * t11116 * t2554;
    (t29694,)
}
