//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 762/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk762(t1062: f64, t5145: f64, t2957: f64, t5129: f64, t2961: f64, t4044: f64, t5066: f64, t5070: f64, t5074: f64, t434: f64, t1542: f64) -> (f64, f64, f64, f64, f64) {
    let t5146 = t5145 * t1062;
    let t5149 = t5129 * t2957;
    let t5156 = t2961 - 0.61805555555555555556e-2_f64 * t4044 - 0.61805555555555555555e-2_f64 * t5066 + 0.18541666666666666667e-1_f64 * t5070 + 0.92708333333333333333e-2_f64 * t5074;
    let t5157 = t5156 * t434;
    let t5161 = t1542 * t1542;
    (t5146, t5149, t5156, t5157, t5161)
}
