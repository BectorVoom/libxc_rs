//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1154/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1154(t10729: f64, t40075: f64, t25172: f64, t3332: f64, t6165: f64, t25177: f64, t7614: f64, t11659: f64, t6395: f64, t10868: f64, t7615: f64, t11714: f64, t6493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40076 = t40075 * t10729;
    let t40081 = t6165 * t3332 * t25172;
    let t40084 = t7614 * t3332 * t25177;
    let t40086 = t6395 * t11659;
    let t40090 = t7614 * t10868 * t7615;
    let t40092 = t6493 * t11714;
    (t40076, t40081, t40084, t40086, t40090, t40092)
}
