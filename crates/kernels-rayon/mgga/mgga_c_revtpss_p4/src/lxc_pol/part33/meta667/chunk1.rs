//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2192/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2192(t108710: f64, t1937: f64, t21881: f64, t94: f64, t29508: f64, t6993: f64, t25082: f64, t86815: f64, t8717: f64, t7003: f64, t27123: f64, t7735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108712 = 2.0_f64 * t108710 * t1937;
    let t108714 = t94 * t21881;
    let t108716 = 2.0_f64 * t108714 * t1937;
    let t108718 = 2.0_f64 * t29508 * t6993;
    let t108721 = 6.0_f64 * t25082 * t8717 * t86815;
    let t108723 = 2.0_f64 * t29508 * t7003;
    let t108725 = 4.0_f64 * t27123 * t7735;
    (t108712, t108716, t108718, t108721, t108723, t108725)
}
