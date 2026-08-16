//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1018/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1018(t1089: f64, t11928: f64, t1071: f64, t1086: f64, t994: f64, t11869: f64, t3316: f64, t989: f64, t1082: f64, t11804: f64, t11239: f64, t11627: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12150 = t11928 * t1089;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12157 = t11869 * t1089;
    let t12160 = t989 * t3316;
    let t12163 = t1082 * t11804;
    let t12166 = t11239 * t11627;
    (t12150, t12154, t12157, t12160, t12163, t12166)
}
