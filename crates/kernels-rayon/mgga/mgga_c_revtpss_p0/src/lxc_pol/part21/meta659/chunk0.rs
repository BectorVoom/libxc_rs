//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2451/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2451(t11880: f64, t3241: f64, t1011: f64, t1016: f64, t2438: f64, t3237: f64, t697: f64, t1010: f64, t10345: f64, t11883: f64, t3244: f64, t11766: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42712 = t3241 * t11880;
    let t42716 = t1011 * t2438 * t1016;
    let t42719 = t1011 * t697 * t3237;
    let t42721 = t10345 * t1010;
    let t42724 = t11883 * t3244;
    let t42727 = t1011 * t140 * t11766;
    (t42712, t42716, t42719, t42721, t42724, t42727)
}
