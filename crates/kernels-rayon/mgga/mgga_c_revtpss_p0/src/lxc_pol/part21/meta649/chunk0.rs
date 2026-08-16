//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2434/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2434(t367: f64, t371: f64, t373: f64, t9291: f64, t1058: f64, t11907: f64, t3197: f64, t3201: f64, t11962: f64, t3231: f64, t11973: f64, t11904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42121 = 0.14820648238345094262e-3_f64 * t367 * t371 * t9291 * t373;
    let t42122 = t11907 * t1058;
    let t42124 = t3197 * t3201;
    let t42139 = t11962 * t1058;
    let t42141 = t3231 * t3201;
    let t42146 = t11973 * t1058;
    let t42149 = t11904 * t1058;
    (t42121, t42122, t42124, t42139, t42141, t42146, t42149)
}
