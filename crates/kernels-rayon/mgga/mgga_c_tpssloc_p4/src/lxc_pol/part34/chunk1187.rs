//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1187/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1187(t22473: f64, t75603: f64, t20342: f64, t6530: f64, t1458: f64, t5449: f64, t1845: f64, t6330: f64, t22633: f64, t22635: f64, t26337: f64, t6460: f64) -> (f64, f64, f64, f64, f64) {
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    let t106956 = t5449 * t1458;
    let t106971 = t6330 * t1845;
    let t106982 = t22633 * t22635 * t26337 * t6460;
    (t106946, t106948, t106956, t106971, t106982)
}
