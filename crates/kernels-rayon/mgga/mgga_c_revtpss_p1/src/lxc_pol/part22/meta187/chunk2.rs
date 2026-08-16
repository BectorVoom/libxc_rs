//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1200/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1200(t1548: f64, t775: f64, t800: f64, t4365: f64, t837: f64, t4364: f64, t125: f64, t1544: f64) -> (f64, f64, f64) {
    let t4442 = t800 * t1548 * t775;
    let t4446 = t4365 * t837;
    let t4447 = t4364 * t4446;
    let t4450 = t125 * t1544;
    (t4442, t4447, t4450)
}
