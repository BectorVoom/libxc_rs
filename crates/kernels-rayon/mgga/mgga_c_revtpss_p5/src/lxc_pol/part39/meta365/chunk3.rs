//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1278/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1278(t15283: f64, t953: f64, t1622: f64, t2944: f64, t1634: f64, t2988: f64, t15127: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64) -> (f64, f64, f64, f64, f64) {
    let t15284 = t15283 * t953;
    let t15287 = t1622 * t2944;
    let t15290 = t1634 * t2988;
    let t15301 = 0.22954444444444444444e0_f64 * t15127;
    let t15312 = 0.27785333333333333334e0_f64 * t15168;
    let t15315 = -0.34431666666666666667e0_f64 * t15137 - 0.57386111111111111112e0_f64 * t15142 + 0.20659e1_f64 * t15147 + 0.103295e1_f64 * t15151 + 0.20659e1_f64 * t15156 - 0.309885e1_f64 * t15160 + 0.20839e0_f64 * t15163 - 0.62517e0_f64 * t15166 - t15312 + 0.46308888888888888889e-1_f64 * t15170 - 0.69463333333333333334e-1_f64 * t15173;
    (t15284, t15287, t15290, t15301, t15315)
}
