//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 346/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk346(t1633: f64, t973: f64, t1598: f64, t1612: f64, t1614: f64, t1622: f64, t1627: f64, t300: f64, t311: f64, t946: f64, t965: f64, t964: f64) -> (f64, f64, f64, f64) {
    let t1634 = t1633 * t973;
    let t1638 = t300 * (-0.310907e-1_f64 * t1614 * t311 + 1.0_f64 * t946 * t1622 + t1598 - t1612 - 0.19751673498613801407e-1_f64 * t1627 + 0.5848223622634646207e0_f64 * t965 * t1634);
    let t1640 = 0.19751673498613801407e-1_f64 * t300 * t1627;
    let t1642 = t964 * t1633 * t973;
    (t1634, t1638, t1640, t1642)
}
