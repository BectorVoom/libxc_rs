//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1117/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1117(t12273: f64, t1992: f64, t6976: f64, t268: f64, t547: f64, t6559: f64, t22705: f64, t22733: f64, t22633: f64, t22694: f64, t3807: f64, t12272: f64) -> (f64, f64, f64, f64, f64) {
    let t81225 = t1992 * t6976 * t12273;
    let t81228 = t6559 * t547 * t268;
    let t81230 = t81228 * t22705 * t22733;
    let t81234 = t22633 * t6976 * t22694 * t3807;
    let t81238 = t22633 * t6976 * t12272 * t3807;
    (t81225, t81228, t81230, t81234, t81238)
}
