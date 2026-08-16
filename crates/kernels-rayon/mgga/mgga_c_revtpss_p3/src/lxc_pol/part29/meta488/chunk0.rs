//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1772/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1772(t1450: f64, t5591: f64, t2013: f64, t8995: f64, t1448: f64, t1907: f64, t4292: f64, t93: f64, t2106: f64, t9593: f64) -> (f64, f64, f64, f64, f64) {
    let t28176 = t1450 * t5591;
    let t28196 = t2013 * t8995;
    let t28198 = t1907 * t1448;
    let t28219 = t93 * t4292;
    let t28286 = t2106 * t9593;
    (t28176, t28196, t28198, t28219, t28286)
}
