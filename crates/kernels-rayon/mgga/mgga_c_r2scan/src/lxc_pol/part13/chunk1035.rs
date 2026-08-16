//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1035/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1035(t2625: f64, t6212: f64, t2634: f64, t2612: f64, t1543: f64, t921: f64, t2531: f64, t481: f64, t113: f64, t7197: f64, t1550: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25486 = t6212 * t2625;
    let t25499 = t6212 * t2634;
    let t25503 = t6212 * t2612;
    let t25562 = t921 * t1543;
    let t25569 = t2531 * t481;
    let t25573 = t7197 * t113;
    let t25577 = t2634 * t481;
    let t25670 = t910 * t1550 * t113;
    (t25486, t25499, t25503, t25562, t25569, t25573, t25577, t25670)
}
