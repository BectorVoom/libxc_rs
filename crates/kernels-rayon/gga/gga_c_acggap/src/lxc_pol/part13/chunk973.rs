//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 973/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk973(t2122: f64, t2131: f64, t2132: f64, t847: f64, t7990: f64, t7994: f64, t2130: f64, t851: f64, t7998: f64, t7987: f64, t7984: f64, t3644: f64, t609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32161 = t2131 * t2132 * t2122 * t847;
    let t32163 = t7990 * t7994;
    let t32165 = t851 * t2130;
    let t32167 = 0.26020884564615598386e1_f64 * t32165 * t7998;
    let t32168 = t7987 * t7998;
    let t32171 = t7990 * t7984;
    let t32176 = 0.8673628188205199462e0_f64 * t2131 * t2132 * t609 * t3644;
    (t32161, t32163, t32167, t32168, t32171, t32176)
}
