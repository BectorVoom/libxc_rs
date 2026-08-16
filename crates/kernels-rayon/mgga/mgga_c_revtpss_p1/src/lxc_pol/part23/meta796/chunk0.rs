//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2618/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2618(t10744: f64, t18409: f64, t808: f64, t18414: f64, t40521: f64, t40791: f64, t5989: f64, t10890: f64, t5985: f64, t14686: f64, t18525: f64, t50570: f64, t61956: f64) -> (f64, f64, f64, f64, f64) {
    let t62069 = t10744 * t808 * t18409;
    let t62072 = t40521 * t808 * t18414;
    let t62089 = t40791 * t5989;
    let t62095 = t10890 * t5985;
    let t62105 = t50570 * t14686 * t61956 * t18525;
    (t62069, t62072, t62089, t62095, t62105)
}
