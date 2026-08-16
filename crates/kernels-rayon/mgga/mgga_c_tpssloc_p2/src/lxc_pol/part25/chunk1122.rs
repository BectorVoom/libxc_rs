//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1122/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1122(t1377: f64, t1385: f64, t22633: f64, t22635: f64, t3719: f64, t22638: f64, t81159: f64, t22892: f64, t6891: f64, t80645: f64, t6892: f64, t81186: f64) -> (f64, f64, f64, f64) {
    let t81346 = t22633 * t22635 * t1377 * t3719 * t1385;
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    (t81346, t81350, t81365, t81375)
}
