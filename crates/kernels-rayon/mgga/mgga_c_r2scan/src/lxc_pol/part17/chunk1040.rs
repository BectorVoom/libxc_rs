//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1040/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1040(t2634: f64, t6212: f64, t2612: f64, t2531: f64, t2599: f64, t3433: f64, t10855: f64, t110: f64, t11747: f64, t545: f64, t146: f64, t6533: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25499 = t6212 * t2634;
    let t25503 = t6212 * t2612;
    let t25737 = t6212 * t2531;
    let t25826 = t3433 * t2599;
    let t25851 = t10855 * t110;
    let t25983 = t545 * t11747;
    let t26088 = t146 * t6533 * t978;
    (t25499, t25503, t25737, t25826, t25851, t25983, t26088)
}
