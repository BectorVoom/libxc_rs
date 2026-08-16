//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1485/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1485(t1518: f64, t2198: f64, t10208: f64, t104: f64, t69: f64, t2339: f64, t2681: f64, t64: f64, t10207: f64, t111: f64, t116: f64, t21813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35858 = t1518 * t2198;
    let t36307 = t10208 * t104;
    let t36308 = t69 * t36307;
    let t36314 = t2339 * t104;
    let t36315 = t69 * t36314;
    let t46089 = t64 * t2681;
    let t46157 = 1.0_f64 / t10207 / t111;
    let t75439 = t21813 * t116;
    (t35858, t36308, t36315, t46089, t46157, t75439)
}
