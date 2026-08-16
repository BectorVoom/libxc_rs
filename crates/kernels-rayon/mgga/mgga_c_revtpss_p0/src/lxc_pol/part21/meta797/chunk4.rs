//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2885/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2885(t324: f64, t52345: f64, t52366: f64, t11507: f64, t1633: f64, t11409: f64, t11410: f64, t1622: f64, t41813: f64, t52153: f64, t52156: f64, t52159: f64, t52162: f64, t52166: f64, t52170: f64, t52174: f64, t52176: f64, t52178: f64, t52180: f64, t52182: f64, t52185: f64, t972: f64) -> (f64, f64) {
    let t52368 = (t52345 + t52366) * t324;
    let t52370 = t11507 * t1633;
    let t52377 = t52153 + t52156 + t52159 - t52162 - t52166 - t52170 - t52174 + t52176 + t52178 - t52180 - t52182 + t52185 - 0.19751673498613801407e-1_f64 * t52368 + 0.30762056574649219974e4_f64 * t52370 * t41813 * t972 - 24.0_f64 * t11409 * t1622 * t11410;
    (t52368, t52377)
}
