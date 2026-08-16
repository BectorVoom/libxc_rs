//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 356/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk356(t1186: f64, t489: f64, t1183: f64, t187: f64, t497: f64, t72: f64, t732: f64, t177: f64) -> (f64, f64, f64, f64, f64) {
    let t1187 = t489 * t1186;
    let t1189 = 0.19751673498613801407e-1_f64 * t1183 * t187;
    let t1190 = t497 * t72;
    let t1192 = 0.18311447306006545054e-3_f64 * t1190 * t732;
    let t1193 = t497 * t177;
    (t1187, t1189, t1190, t1192, t1193)
}
