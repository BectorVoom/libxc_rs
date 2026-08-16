//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 892/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk892(t6205: f64, t973: f64, t3014: f64, t6189: f64, t1622: f64, t1634: f64, t2943: f64, t2968: f64, t2987: f64, t3012: f64, t311: f64, t4647: f64, t4685: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6152: f64, t6158: f64, t6174: f64, t6177: f64, t6185: f64, t6190: f64, t946: f64, t965: f64) -> (f64, f64, f64) {
    let t6206 = t6205 * t973;
    let t6209 = t6189 * t3014;
    let t6212 = -0.310907e-1_f64 * t6152 * t311 + 2.0_f64 * t4647 * t1622 - 2.0_f64 * t2943 * t6158 + 1.0_f64 * t946 * t6174 + 0.32163958997385070134e2_f64 * t2968 * t6177 + t6106 - t6108 + t6112 - t6144 - t6147 - 0.19751673498613801407e-1_f64 * t6185 + 0.11696447245269292414e1_f64 * t4685 * t1634 - 0.11696447245269292414e1_f64 * t2987 * t6190 + 0.5848223622634646207e0_f64 * t965 * t6206 + 0.17315859105681463759e2_f64 * t3012 * t6209;
    (t6206, t6209, t6212)
}
