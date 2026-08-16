//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2080/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2080(t1078: f64, t1982: f64, t3140: f64, t4930: f64, t25604: f64, t7825: f64, t1678: f64, t7150: f64, t8521: f64, t27418: f64, t3057: f64, t3046: f64, t7810: f64) -> (f64, f64, f64, f64, f64) {
    let t99886 = t1982 * t4930 * t3140 * t1078;
    let t99909 = t7825 * t25604;
    let t99914 = t7150 * t1678;
    let t99915 = t99914 * t8521;
    let t99934 = t3057 * t27418;
    let t99940 = t3046 * t7810;
    (t99886, t99909, t99915, t99934, t99940)
}
