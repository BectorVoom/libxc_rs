//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2205/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2205(t1518: f64, t1936: f64, t2371: f64, t572: f64, t670: f64, t7002: f64, t4158: f64, t7953: f64, t101469: f64, t117: f64, t2327: f64, t7741: f64) -> (f64, f64, f64, f64, f64) {
    let t101590 = 6.0_f64 * t572 * t2371 * t1936 * t1518;
    let t101594 = 12.0_f64 * t572 * t670 * t7002 * t1518;
    let t101598 = 3.0_f64 * t4158 * t7953;
    let t101601 = 3.0_f64 * t572 * t117 * t101469;
    let t101606 = 6.0_f64 * t572 * t2327 * t7741;
    (t101590, t101594, t101598, t101601, t101606)
}
