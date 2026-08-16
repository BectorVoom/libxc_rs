//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 841/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk841(t322: f64, t945: f64, t174: f64, t361: f64, t157: f64, t406: f64, t864: f64, t1016: f64, t965: f64, t1487: f64, t435: f64, t929: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15407 = t945 * t322;
    let t15695 = t361 * t174;
    let t15758 = t864 * t406 * t157;
    let t15897 = t965 * t1016;
    let t15995 = t435 * t1487;
    let t16020 = t322 * t929 * t157;
    (t15407, t15695, t15758, t15897, t15995, t16020)
}
