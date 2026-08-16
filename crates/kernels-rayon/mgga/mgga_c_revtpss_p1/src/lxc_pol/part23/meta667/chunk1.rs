//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2400/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2400(t1014: f64, t11150: f64, t1003: f64, t11735: f64, t221: f64, t345: f64, t346: f64, t624: f64, t11858: f64, t16048: f64, t1065: f64, t215: f64) -> (f64, f64, f64, f64, f64) {
    let t42731 = t1014 * t11150;
    let t42740 = t1003 * t11735;
    let t42745 = 5.0_f64 / 486.0_f64 * t345 * t221 * t624 * t346;
    let t42765 = t11858 * t16048;
    let t42778 = t215 * t1065;
    (t42731, t42740, t42745, t42765, t42778)
}
