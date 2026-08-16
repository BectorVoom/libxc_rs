//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 901/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk901(t1413: f64, t1449: f64, t2481: f64, t2484: f64, t2507: f64, t2510: f64, t430: f64, t453: f64, t459: f64, t4769: f64, t4772: f64, t4828: f64, t6631: f64, t6634: f64, t6639: f64, t6642: f64, t6645: f64, t6692: f64, t6700: f64, t6703: f64, t6706: f64, t6709: f64, t6712: f64, t6747: f64, t987: f64, t995: f64) -> f64 {
    let t6750 = 0.165625e-1_f64 * t6631 * t987 - 0.6625e-1_f64 * t6634 * t2484 + 0.33125e-1_f64 * t2481 * t2507 + 0.99375e-1_f64 * t4772 * t6639 - 0.6625e-1_f64 * t1413 * t6642 - 0.33125e-1_f64 * t1413 * t6645 + 0.165625e-1_f64 * t430 * t6692 - 0.33125e-1_f64 * t4769 * t995 + 0.19875e0_f64 * t4772 * t2510 * t459 - 0.6625e-1_f64 * t1413 * t6700 - 0.33125e-1_f64 * t1413 * t6703 - 0.19875e0_f64 * t4828 * t6706 + 0.99375e-1_f64 * t1449 * t6709 + 0.496875e-1_f64 * t1449 * t6712 - 0.165625e-1_f64 * t453 * t6747;
    t6750
}
