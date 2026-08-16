//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1576/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576(t22865: f64, t9918: f64, t1883: f64, t6883: f64, t9816: f64, t9818: f64, t13999: f64, t22833: f64, t22813: f64, t547: f64, t807: f64, t9941: f64) -> (f64, f64, f64, f64) {
    let t86112 = t9918 * t22865;
    let t86124 = t9816 * t9818 * t6883 * t1883;
    let t86156 = t13999 * t22833;
    let t86165 = t807 * t547 * t9941 * t22813;
    (t86112, t86124, t86156, t86165)
}
