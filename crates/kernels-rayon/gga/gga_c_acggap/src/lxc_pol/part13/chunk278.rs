//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 278/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk278(t1049: f64, t347: f64, t136: f64, t357: f64, t576: f64, t137: f64, t154: f64, t922: f64, t345: f64, t125: f64, t134: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1050 = t1049 * t347;
    let t1053 = t576 * t136 * t357;
    let t1054 = t1053 / 6.0_f64;
    let t1055 = t154 * t137;
    let t1056 = t1055 * t922;
    let t1057 = t345 * t1056;
    let t1059 = t134 * t125;
    let t1060 = t352 * t1059;
    (t1050, t1053, t1054, t1055, t1056, t1057, t1059, t1060)
}
