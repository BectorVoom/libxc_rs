//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 701/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk701(t684: f64, t7114: f64, t15312: f64, t24873: f64, t4255: f64, t10703: f64, t11593: f64, t1901: f64, t29147: f64, t29151: f64, t29155: f64, t29158: f64, t29162: f64, t29166: f64, t29170: f64, t29174: f64, t29178: f64, t29182: f64, t29186: f64, t3281: f64, t446: f64) -> (f64, f64, f64) {
    let t29189 = t7114 * t684;
    let t29190 = t15312 * t29189;
    let t29193 = t24873 * t4255;
    let t29194 = t10703 * t29193;
    let t29197 = t1901 * t29147 / 9.0_f64 + t1901 * t29151 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11593 * t29155 - t446 * t29158 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t3281 * t29162 - t446 * t29166 / 9.0_f64 - t446 * t29170 / 3.0_f64 - t446 * t29174 / 3.0_f64 - t446 * t29178 / 3.0_f64 - t446 * t29182 / 3.0_f64 - t1901 * t29186 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t29190 - t1901 * t29194 / 9.0_f64;
    (t29189, t29193, t29197)
}
