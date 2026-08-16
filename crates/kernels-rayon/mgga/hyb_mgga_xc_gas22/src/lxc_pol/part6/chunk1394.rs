//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1394/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1394(t1005: f64, t3579: f64, t1434: f64, t25432: f64, t25436: f64, t260: f64, t29598: f64, t29627: f64, t29629: f64, t29631: f64, t29633: f64, t29635: f64, t29637: f64, t29639: f64, t29640: f64, t29644: f64, t29648: f64, t29694: f64, t29741: f64, t29988: f64, t30045: f64, t30098: f64, t30216: f64, t3583: f64) -> (f64, f64) {
    let t30221 = t3579 * t1005;
    let t30228 = t260 * (t29598 + t29640 + t29694 + t29741 + t29988 + t30045 + t30098 + t30216) + 0.4155806185363551302e3_f64 * t25436 * t3583 * t30221 - 0.14035736694323150897e2_f64 * t25432 * t1434 * t30221 - t29627 + t29629 + t29631 - t29633 - t29635 - t29637 + t29639 - t29644 + t29648;
    (t30221, t30228)
}
