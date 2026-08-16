//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2265/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265(t1936: f64, t98484: f64, t98487: f64, t27123: f64, t7002: f64, t13514: f64, t93: f64, t101469: f64, t1312: f64, t28219: f64, t25832: f64, t7889: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101517 = 2.0_f64 * t98484 * t1936;
    let t101519 = 4.0_f64 * t98487 * t1936;
    let t101521 = 4.0_f64 * t27123 * t7002;
    let t101522 = t93 * t13514;
    let t101524 = 2.0_f64 * t101522 * t1936;
    let t101526 = 2.0_f64 * t1312 * t101469;
    let t101528 = 4.0_f64 * t28219 * t7002;
    let t101530 = 2.0_f64 * t7889 * t25832;
    (t101517, t101519, t101521, t101524, t101526, t101528, t101530)
}
