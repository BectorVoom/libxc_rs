//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1166/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1166(t108379: f64, t7515: f64, t30226: f64, t689: f64, t94768: f64, t94763: f64, t108279: f64, t22453: f64, t96463: f64, t213: f64, t30247: f64, t6896: f64, t7492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109609 = t108379 * t7515;
    let t109630 = t30226 * t689;
    let t109631 = t94768 * t109630;
    let t109633 = t94763 * t109630;
    let t109647 = t108279 * t7515;
    let t109651 = t96463 * t22453;
    let t109706 = t213 * t30247;
    let t109715 = t689 * t7492 * t6896;
    (t109609, t109631, t109633, t109647, t109651, t109706, t109715)
}
