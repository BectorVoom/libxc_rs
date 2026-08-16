//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1041/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1041(t120006: f64, t120151: f64, t2453: f64, t31798: f64, t119974: f64, t25304: f64, t1035: f64, t1052: f64, t8514: f64, t8515: f64, t3058: f64, t31991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120152 = t120151 * t120006;
    let t120154 = t2453 * t31798;
    let t120156 = 0.95199562775170587692e-3_f64 * t120154 * t119974;
    let t120157 = t25304 * t31798;
    let t120159 = 0.50779446784275991476e-2_f64 * t120157 * t119974;
    let t120179 = t1035 * t1052;
    let t120181 = t8514 * t8515 * t120179;
    let t120184 = t3058 * t31991;
    (t120152, t120156, t120159, t120179, t120181, t120184)
}
