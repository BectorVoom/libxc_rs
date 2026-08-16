//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1011/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1011(t26414: f64, t6976: f64, t22633: f64, t5345: f64, t1992: f64, t1799: f64, t562: f64, t1352: f64, t22705: f64, t7736: f64, t22704: f64, t6883: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26415 = t6976 * t26414;
    let t26416 = t22633 * t26415;
    let t26418 = t6976 * t5345;
    let t26419 = t1992 * t26418;
    let t26421 = t562 * t1799;
    let t26422 = t26421 * t1352;
    let t26423 = t6976 * t26422;
    let t26424 = t22633 * t26423;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    (t26416, t26419, t26421, t26424, t26427, t26429)
}
