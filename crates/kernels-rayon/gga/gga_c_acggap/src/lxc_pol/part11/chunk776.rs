//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 776/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk776(t7932: f64, t8406: f64, t7942: f64, t2341: f64, t463: f64, t8004: f64, t2147: f64, t2331: f64, t322: f64, t2132: f64, t2138: f64, t309: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8407 = t7932 * t8406;
    let t8408 = t7942 * t8407;
    let t8410 = t2341 * t463;
    let t8411 = t8004 * t8410;
    let t8415 = t2147 * t2331 * t463;
    let t8418 = t2331 * t322;
    let t8419 = t2132 * t8418;
    let t8420 = t2138 * t8419;
    let t8422 = t2331 * t309;
    (t8407, t8408, t8411, t8415, t8418, t8419, t8420, t8422)
}
