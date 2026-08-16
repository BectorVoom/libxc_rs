//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1039/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1039(t1936: f64, t27060: f64, t29432: f64, t7002: f64, t7586: f64, t32165: f64, t32167: f64, t32169: f64, t32172: f64, t32174: f64, t32176: f64, t32178: f64, t32815: f64, t32825: f64, t670: f64, t8564: f64) -> f64 {
    let t32828 = t27060 * t1936;
    let t32830 = t29432 * t1936;
    let t32832 = t7586 * t7002;
    let t32837 = 2.0_f64 * t32825 * t670 + 2.0_f64 * t32165 + 2.0_f64 * t32167 + 2.0_f64 * t32169 + t32172 + t32174 + t32176 + t32178 + t32815 + 2.0_f64 * t32828 + 2.0_f64 * t32830 + 2.0_f64 * t32832 + t8564;
    t32837
}
