//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2079/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079(t26093: f64, t575: f64, t116: f64, t25832: f64, t26133: f64, t571: f64, t2327: f64, t7724: f64, t27833: f64, t7316: f64, t13426: f64, t7003: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95127 = t26093 * t575;
    let t95137 = t116 * t25832;
    let t95180 = t571 * t26133;
    let t97593 = t7724 * t2327;
    let t97604 = 2.0_f64 * t27833 * t7316;
    let t97606 = 4.0_f64 * t13426 * t7003;
    (t95127, t95137, t95180, t97593, t97604, t97606)
}
