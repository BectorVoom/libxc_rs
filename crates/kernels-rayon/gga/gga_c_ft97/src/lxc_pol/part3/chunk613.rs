//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 613/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk613(t2599: f64, t5166: f64, t1168: f64, t992: f64, t2607: f64, t2606: f64, t265: f64, t4969: f64, t724: f64, t1901: f64, t193: f64, t2553: f64, t3835: f64, t3958: f64, t3986: f64, t3988: f64, t446: f64, t5066: f64, t5070: f64, t5075: f64, t5079: f64, t5083: f64, t5087: f64, t5134: f64, t5149: f64, t5153: f64, t5157: f64, t5161: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t5167 = t2599 * t5166;
    let t5170 = t992 * t1168;
    let t5171 = t2607 * t5170;
    let t5172 = t2606 * t5171;
    let t5176 = t724 * t265 * t4969;
    let t5179 = -2.0_f64 / 9.0_f64 * t3958 + 2.0_f64 / 3.0_f64 * t446 * t5066 + 2.0_f64 / 3.0_f64 * t446 * t5070 + 2.0_f64 / 3.0_f64 * t446 * t5075 - 2.0_f64 / 9.0_f64 * t446 * t5079 - t446 * t5083 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t5087 + 2.0_f64 / 9.0_f64 * t3986 + t2553 + 2.0_f64 / 9.0_f64 * t3988 + t89 * t193 * t5134 / 3.0_f64 - t446 * t5149 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t5153 - 2.0_f64 / 3.0_f64 * t446 * t5157 - t446 * t5161 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t3835 + 2.0_f64 / 9.0_f64 * t1901 * t5167 + 2.0_f64 / 9.0_f64 * t1901 * t5172 + 2.0_f64 / 9.0_f64 * t446 * t5176;
    (t5167, t5171, t5172, t5176, t5179)
}
