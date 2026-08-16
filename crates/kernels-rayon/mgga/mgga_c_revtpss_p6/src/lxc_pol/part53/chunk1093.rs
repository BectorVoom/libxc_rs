//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1093/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1093(t119934: f64, t31752: f64, t31758: f64, t119857: f64, t1955: f64, t136: f64, t233: f64, t2457: f64, t2453: f64, t31778: f64, t25304: f64, t119813: f64, t31799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119935 = t31752 * t119934;
    let t119936 = t119935 * t31758;
    let t119941 = t1955 * t119857;
    let t119955 = t233 * t136 * t2457;
    let t119957 = 0.3427046870806409921e-2_f64 * t2453 * t31778 * t119955;
    let t119960 = 0.45699670022203476294e-2_f64 * t25304 * t31778 * t119955;
    let t119966 = 0.19039912555034117539e-1_f64 * t31799 * t119813;
    (t119935, t119936, t119941, t119957, t119960, t119966)
}
