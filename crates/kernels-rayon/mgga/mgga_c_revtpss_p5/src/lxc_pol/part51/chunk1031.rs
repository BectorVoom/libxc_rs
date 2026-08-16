//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1031/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1031(t122: f64, t2466: f64, t31780: f64, t119928: f64, t240: f64, t822: f64, t843: f64, t31752: f64, t31758: f64, t119857: f64, t1955: f64, t136: f64, t233: f64, t2457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119930 = t31780 * t122 * t2466;
    let t119931 = t119928 * t119930;
    let t119934 = t822 * t843 * t240;
    let t119935 = t31752 * t119934;
    let t119936 = t119935 * t31758;
    let t119941 = t1955 * t119857;
    let t119955 = t233 * t136 * t2457;
    (t119930, t119931, t119935, t119936, t119941, t119955)
}
