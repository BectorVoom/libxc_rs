//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 910/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk910(t30402: f64, t30407: f64, t30409: f64, t372: f64, t141: f64, t7335: f64, t301: f64, t7325: f64, t1016: f64, t1072: f64, t30418: f64, t2019: f64, t2028: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31095 = t30407 * t30402 * t30409 * t372;
    let t31097 = t7335 * t141;
    let t31100 = t30407 * t31097 * t7325 * t301;
    let t31102 = t1016 * t1072;
    let t31105 = t30407 * t30418 * t31102 * t372;
    let t31110 = t2019 * t2028;
    (t31095, t31097, t31100, t31102, t31105, t31110)
}
