//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1584/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1584(t10631: f64, t808: f64, t10886: f64, t2699: f64, t798: f64, t802: f64, t2703: f64, t2707: f64, t10489: f64, t124: f64, t800: f64, t159: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10887 = t808 * t10631;
    let t10888 = t10886 * t10887;
    let t10890 = t2699 * t798;
    let t10891 = t10890 * t802;
    let t10893 = t2703 * t2707;
    let t10895 = t124 * t10489;
    let t10896 = t800 * t10895;
    let t10899 = t159 * t853;
    (t10887, t10888, t10890, t10891, t10893, t10896, t10899)
}
