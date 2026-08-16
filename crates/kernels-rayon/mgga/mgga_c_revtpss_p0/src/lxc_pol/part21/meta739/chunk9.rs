//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2601/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2601(t10175: f64, t14079: f64, t10073: f64, t13731: f64, t3915: f64, t5721: f64, t9288: f64, t2439: f64, t3895: f64, t5775: f64, t14066: f64, t213: f64) -> (f64, f64, f64, f64, f64) {
    let t47893 = t10175 * t14079;
    let t47899 = t10073 * t13731;
    let t47904 = t3915 * t5721 * t9288;
    let t47907 = t2439 * t3895 * t5775;
    let t47909 = t213 * t14066;
    (t47893, t47899, t47904, t47907, t47909)
}
