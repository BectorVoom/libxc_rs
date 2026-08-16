//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 469/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk469(t357: f64, t1038: f64, t1052: f64, t1036: f64, t1033: f64, t127: f64, t246: f64, t1046: f64, t1041: f64, t283: f64, t905: f64, t1020: f64, t1062: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3154 = t357 * t357;
    let t3167 = t1052 * t1038;
    let t3168 = t1036 * t3167;
    let t3169 = t1033 * t3168;
    let t3172 = t246 * t127;
    let t3173 = t3172 * t1046;
    let t3174 = t1041 * t3173;
    let t3181 = 1.0_f64 / t283 / t905;
    let t3188 = t1020 * t1062;
    (t3154, t3169, t3172, t3174, t3181, t3188)
}
