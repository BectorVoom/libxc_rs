//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 929/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk929(t5092: f64, t9890: f64, t747: f64, t91: f64, t3902: f64, t3938: f64, t18168: f64, t18171: f64, t18174: f64, t10119: f64, t14005: f64, t18153: f64, t18157: f64, t18162: f64, t18165: f64) -> (f64, f64, f64) {
    let t18370 = t9890 * t5092;
    let t18372 = t91 * t18370 * t747;
    let t18375 = t91 * t3902 * t3938;
    let t18381 = t18168 / 9.0_f64;
    let t18382 = 2.0_f64 / 9.0_f64 * t18171;
    let t18383 = 2.0_f64 / 27.0_f64 * t18174;
    let t18384 = 3.0_f64 / 8.0_f64 * t18372 - t18375 / 2.0_f64 + 2.0_f64 * t18153 - t18157 / 3.0_f64 - 6.0_f64 * t18162 + 4.0_f64 * t18165 + t18381 - t18382 + t18383 - t10119 - t14005;
    (t18372, t18375, t18384)
}
