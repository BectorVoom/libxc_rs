//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 525/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk525(t1181: f64, t3169: f64, t388: f64, t1163: f64, t322: f64, t435: f64, t157: f64, t372: f64, t406: f64) -> (f64, f64, f64, f64) {
    let t3171 = t1181 * t388 * t3169;
    let t3172 = t1163 * t3171;
    let t3174 = t435 * t322;
    let t3176 = t372 * t406 * t157;
    (t3171, t3172, t3174, t3176)
}
