//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2207/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2207(t1977: f64, t3057: f64, t1078: f64, t11200: f64, t7143: f64, t1651: f64, t988: f64, t15827: f64, t27536: f64, t15904: f64, t25515: f64, t12047: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99953 = t3057 * t1977;
    let t99969 = t11200 * t7143 * t1078;
    let t99970 = t1651 * t988;
    let t99983 = 0.11433071498151929859e-2_f64 * t27536 * t15827;
    let t99984 = t25515 * t15904;
    let t99985 = t12047 * t99984;
    (t99953, t99969, t99970, t99983, t99984, t99985)
}
