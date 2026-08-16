//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 662/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk662(t1062: f64, t4142: f64, t1530: f64, t2957: f64, t1061: f64, t2836: f64, t2961: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64, t434: f64) -> (f64, f64, f64, f64, f64) {
    let t4143 = t4142 * t1062;
    let t4146 = t1530 * t2957;
    let t4147 = t4146 * t1061;
    let t4155 = t2961 - 0.30902777777777777778e-2_f64 * t2836 - 0.30902777777777777778e-2_f64 * t4044 - 0.61805555555555555555e-2_f64 * t4049 + 0.18541666666666666667e-1_f64 * t4054 + 0.92708333333333333333e-2_f64 * t4058;
    let t4156 = t4155 * t434;
    (t4143, t4146, t4147, t4155, t4156)
}
