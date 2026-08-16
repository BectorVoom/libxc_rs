//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1363/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1363(t1410: f64, t2478: f64, t9167: f64, t25267: f64, t3482: f64, t2479: f64, t2521: f64, t4270: f64, t4296: f64, t7061: f64, t11107: f64, t11116: f64, t2538: f64, t2539: f64, t2554: f64, t2560: f64, t29644: f64, t29648: f64, t29650: f64, t29652: f64, t29654: f64, t29656: f64, t29658: f64, t29660: f64, t29663: f64, t29666: f64, t3550: f64, t4297: f64, t7002: f64, t7059: f64, t9241: f64) -> (f64, f64, f64, f64) {
    let t29669 = 4.0_f64 * t2478 * t1410 * t9167;
    let t29671 = 8.0_f64 * t25267 * t3482;
    let t29674 = 6.0_f64 * t2521 * t4270 * t2479;
    let t29684 = t4296 * t7061;
    let t29694 = t29644 - t29648 - t29650 - t29652 + t29654 + t29656 - t29658 - t29660 - t29663 - t29666 + t29669 + t29671 - t29674 - 2.0_f64 * t2538 * t4297 * t2554 - 0.19298375398431042081e3_f64 * t7002 * t11107 * t2539 + 0.32163958997385070134e2_f64 * t2560 * t11107 * t2554 + 0.2069040516770936012e4_f64 * t7059 * t29684 * t2539 + 0.64327917994770140268e2_f64 * t2560 * t3550 * t9241 + 0.2069040516770936012e4_f64 * t7059 * t11116 * t2554;
    (t29669, t29671, t29674, t29694)
}
