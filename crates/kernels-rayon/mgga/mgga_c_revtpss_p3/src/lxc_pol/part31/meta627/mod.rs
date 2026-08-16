//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2080;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta627(t1078: f64, t1982: f64, t3140: f64, t4930: f64, t25604: f64, t7825: f64, t1678: f64, t7150: f64, t8521: f64, t27418: f64, t3057: f64, t3046: f64, t7810: f64, t27543: f64, t994: f64, t1977: f64, t11200: f64, t7143: f64, t15827: f64, t27536: f64, t15904: f64, t25515: f64, t12047: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99886, t99909, t99915, t99934, t99940) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2080(t1078, t1982, t3140, t4930, t25604, t7825, t1678, t7150, t8521, t27418, t3057, t3046, t7810);
        let (t99947, t99953, t99969, t99983, t99984, t99985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2081(t27543, t994, t1977, t3057, t1078, t11200, t7143, t15827, t27536, t15904, t25515, t12047);
    (t99886, t99909, t99915, t99934, t99940, t99947, t99953, t99969, t99983, t99984, t99985)
}
