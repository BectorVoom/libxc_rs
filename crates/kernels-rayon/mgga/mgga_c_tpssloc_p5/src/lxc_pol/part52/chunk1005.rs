//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1005/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1005(t7510: f64, t814: f64, t829: f64, t7528: f64, t794: f64, t6562: f64, t1509: f64, t1902: f64, t1510: f64, t22992: f64, t13380: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25255 = t814 * t7510;
    let t25256 = t25255 * t829;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    let t25262 = t25261 * t829;
    let t25269 = t22992 * t1510;
    let t25272 = t13380 * t232;
    (t25256, t25259, t25261, t25262, t25269, t25272)
}
