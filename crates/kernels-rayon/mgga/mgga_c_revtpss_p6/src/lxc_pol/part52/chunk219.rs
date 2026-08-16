//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 219/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk219(t281: f64, t283: f64, t926: f64, t240: f64, t346: f64, t906: f64, t141: f64, t908: f64, t919: f64, t921: f64, t924: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t928 = t281 * t926 * t283;
    let t929 = 0.82156666666666666667e-1_f64 * t928;
    let t930 = t240 * t346;
    let t931 = t930 * t906;
    let t932 = t141 * t931;
    let t934 = 0.1898925e1_f64 * t919 - t921 - 0.29896666666666666667e0_f64 * t908 + 0.3071625e0_f64 * t924 - t929 - 0.82156666666666666667e-1_f64 * t932;
    let t935 = 1.0_f64 / t290;
    (t928, t929, t930, t931, t932, t934, t935)
}
