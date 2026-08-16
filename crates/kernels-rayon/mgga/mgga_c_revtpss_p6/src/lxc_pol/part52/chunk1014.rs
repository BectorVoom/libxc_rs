//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1014/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1014(t31756: f64, t4364: f64, t837: f64, t31755: f64, t1955: f64, t843: f64, t8464: f64, t8468: f64, t233: f64, t239: f64, t240: f64, t31752: f64) -> (f64, f64, f64, f64) {
    let t31758 = t4364 * t31756 * t837;
    let t31759 = t31755 * t31758;
    let t31763 = t1955 * t8464 * t843 * t8468;
    let t31767 = t31752 * t233 * t239 * t240;
    (t31758, t31759, t31763, t31767)
}
