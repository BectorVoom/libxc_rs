//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1395/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1395(t1014: f64, t21601: f64, t29650: f64, t29652: f64, t29654: f64, t29656: f64, t29658: f64, t29660: f64, t29663: f64, t29666: f64, t29669: f64, t29671: f64, t29674: f64, t3591: f64, t4310: f64, t9001: f64, t9002: f64) -> f64 {
    let t30235 = t29650 + t29652 - t29654 - t29656 + t29658 + t29660 + t29663 + t29666 - t29669 - t29671 + t29674 + 0.12304822629859687989e5_f64 * t1014 * t21601 * t4310 * t9001 - 0.20508037716432813315e4_f64 * t3591 * t9002;
    t30235
}
