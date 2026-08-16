//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2371/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2371(t17730: f64, t5051: f64, t3626: f64, t3566: f64, t489: f64, t17728: f64) -> (f64, f64, f64, f64) {
    let t17731 = t5051 * t17730;
    let t17732 = t3626 * t17731;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    (t17731, t17732, t17735, t17736)
}
