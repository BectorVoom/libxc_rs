//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1231/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1231(t4606: f64, t916: f64, t1600: f64, t2897: f64, t918: f64, t923: f64, t1606: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t4607 = t916 * t4606;
    let t4614 = t2897 * t1600;
    let t4615 = t4614 * t918;
    let t4617 = t923 * t4606;
    let t4620 = t698 * t1606;
    (t4607, t4614, t4615, t4617, t4620)
}
