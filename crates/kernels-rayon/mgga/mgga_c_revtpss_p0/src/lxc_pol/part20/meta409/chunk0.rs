//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1513/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1513(t1065: f64, t215: f64, t1063: f64, t247: f64, t906: f64, t11986: f64, t2858: f64, t11744: f64, t3106: f64, t373: f64, t675: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t42778 = t215 * t1065;
    let t42781 = t1063 * t247 * t42778 * t906;
    let t42785 = t1063 * t247 * t11986 * t2858;
    let t42788 = t3106 * t11744;
    let t42792 = t675 * t373;
    let t42793 = t828 * t42792;
    (t42781, t42785, t42788, t42793)
}
