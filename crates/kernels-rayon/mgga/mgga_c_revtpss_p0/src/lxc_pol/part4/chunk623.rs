//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 623/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk623(t2988: f64, t3014: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t2935: f64, t2938: f64, t2943: f64, t2945: f64, t2963: f64, t2968: f64, t2971: f64, t2980: f64, t2982: f64, t2987: f64, t2989: f64, t3007: f64, t3012: f64, t311: f64, t946: f64, t955: f64, t965: f64, t974: f64) -> (f64, f64) {
    let t3015 = t2988 * t3014;
    let t3018 = -0.310907e-1_f64 * t2935 * t311 + 2.0_f64 * t2938 * t955 - 2.0_f64 * t2943 * t2945 + 1.0_f64 * t946 * t2963 + 0.32163958997385070134e2_f64 * t2968 * t2971 + t2868 - t2871 + t2878 - t2921 - t2929 - 0.19751673498613801407e-1_f64 * t2980 + 0.11696447245269292414e1_f64 * t2982 * t974 - 0.11696447245269292414e1_f64 * t2987 * t2989 + 0.5848223622634646207e0_f64 * t965 * t3007 + 0.17315859105681463759e2_f64 * t3012 * t3015;
    (t3015, t3018)
}
