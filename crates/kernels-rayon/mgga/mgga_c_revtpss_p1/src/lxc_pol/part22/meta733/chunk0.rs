//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2792/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2792(t2693: f64, t2710: f64, t9732: f64, t2430: f64, t853: f64, t2682: f64, t820: f64, t823: f64) -> (f64, f64, f64) {
    let t40535 = t2710 * t9732 * t2693;
    let t40555 = t853 * t2430;
    let t40593 = t820 * t823 * t2682;
    (t40535, t40555, t40593)
}
