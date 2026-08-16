//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1235/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1235(t15669: f64, t1976: f64, t25604: f64, t7825: f64, t1678: f64, t7150: f64, t8521: f64, t27418: f64, t3057: f64, t15731: f64, t7122: f64, t15925: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99721 = t15669 * t1976;
    let t99909 = t7825 * t25604;
    let t99914 = t7150 * t1678;
    let t99915 = t99914 * t8521;
    let t99934 = t3057 * t27418;
    let t100002 = t7122 * t15731;
    let t100025 = t15925 * t25516;
    (t99721, t99909, t99915, t99934, t100002, t100025)
}
