//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 774/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk774(t1034: f64, t3182: f64, t828: f64, t3316: f64, t994: f64, t126: f64, t373: f64) -> (f64, f64, f64, f64) {
    let t11626 = t1034 * t1034;
    let t11627 = 1.0_f64 / t11626;
    let t11703 = t828 * t3182;
    let t11874 = t994 * t3316;
    let t11921 = t126 * t373;
    (t11627, t11703, t11874, t11921)
}
