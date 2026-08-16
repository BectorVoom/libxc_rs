//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1372/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1372(t44545: f64, t828: f64, t3566: f64, t3766: f64, t5330: f64, t1209: f64, t13141: f64, t17708: f64, t371: f64, t481: f64, t482: f64, t9291: f64) -> (f64, f64, f64, f64) {
    let t44546 = t828 * t44545;
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44578 = t1209 * t13141 * t17708;
    let t44607 = 0.14820648238345094262e-3_f64 * t481 * t371 * t9291 * t482;
    (t44546, t44551, t44578, t44607)
}
