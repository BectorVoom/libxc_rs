//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 688/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk688(t2440: f64, t780: f64, t2439: f64, t212: f64, t860: f64) -> (f64, f64, f64) {
    let t2441 = t2440 * t780;
    let t2443 = 0.65049603595885220126e-3_f64 * t2439 * t2441;
    let t2444 = t212 * t860;
    (t2441, t2443, t2444)
}
