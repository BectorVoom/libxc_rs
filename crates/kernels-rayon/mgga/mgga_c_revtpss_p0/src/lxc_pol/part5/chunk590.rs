//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 590/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk590(t3153: f64, t3154: f64, t1036: f64, t3148: f64, t3141: f64, t357: f64, t1038: f64, t1052: f64, t1033: f64, t127: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3155 = t3153 * t3154;
    let t3160 = t1036 * t3148;
    let t3161 = t3141 * t3160;
    let t3162 = t3153 * t357;
    let t3167 = t1052 * t1038;
    let t3168 = t1036 * t3167;
    let t3169 = t1033 * t3168;
    let t3172 = t246 * t127;
    (t3155, t3160, t3161, t3162, t3168, t3169, t3172)
}
