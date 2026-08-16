//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1724/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1724(t1401: f64, t9909: f64, t4000: f64, t820: f64, t843: f64, t4006: f64, t136: f64, t4011: f64) -> (f64, f64, f64, f64) {
    let t9910 = t9909 * t1401;
    let t9918 = t820 * t4000 * t843;
    let t9919 = t9918 * t4006;
    let t9921 = t4011 * t136;
    (t9910, t9918, t9919, t9921)
}
