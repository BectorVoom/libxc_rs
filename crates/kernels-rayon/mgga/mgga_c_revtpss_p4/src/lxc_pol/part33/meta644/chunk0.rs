//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2093/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2093(t26865: f64, t370: f64, t17727: f64, t17423: f64, t29097: f64, t17789: f64, t29100: f64, t17416: f64, t7624: f64, t17608: f64, t7617: f64, t17217: f64, t26880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104646 = t26865 * t370;
    let t104647 = t17727 * t104646;
    let t104651 = 0.11433071498151929859e-2_f64 * t29097 * t17423;
    let t104653 = 0.57165357490759649296e-3_f64 * t29100 * t17789;
    let t104658 = t7624 * t17416;
    let t104677 = t17608 * t7617;
    let t104680 = t26880 * t17217;
    (t104646, t104647, t104651, t104653, t104658, t104677, t104680)
}
