//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 911/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk911(t3129: f64, t3172: f64, t3127: f64, t3135: f64, t1041: f64, t1065: f64, t3059: f64, t906: f64, t1042: f64, t1024: f64, t3105: f64, t3151: f64, t3153: f64) -> (f64, f64, f64, f64, f64) {
    let t11643 = t3172 * t3129;
    let t11644 = t3127 * t11643;
    let t11648 = t3172 * t3135;
    let t11649 = t1041 * t11648;
    let t11651 = t1065 * t3059;
    let t11652 = t11651 * t906;
    let t11653 = t1042 * t11652;
    let t11656 = t1024 * t3105;
    let t11659 = t3151 * t3153;
    (t11644, t11649, t11653, t11656, t11659)
}
