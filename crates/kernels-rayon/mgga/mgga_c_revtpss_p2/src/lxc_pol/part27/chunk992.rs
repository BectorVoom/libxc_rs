//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 992/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk992(t12039: f64, t3269: f64, t11804: f64, t996: f64, t1035: f64, t11239: f64, t342: f64, t11247: f64, t378: f64, t3145: f64, t334: f64, t11249: f64) -> (f64, f64, f64, f64, f64) {
    let t12040 = t3269 * t12039;
    let t12043 = t996 * t11804;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12048 = t378 * t11247;
    let t12050 = 1.0_f64 / t3145 / t334;
    let t12051 = t11249 * t12050;
    (t12040, t12043, t12047, t12048, t12051)
}
