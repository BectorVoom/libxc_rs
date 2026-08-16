//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 312/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk312(t1049: f64, t349: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64, t1010: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1050 = t349 * t1049;
    let t1052 = t382 * t225;
    let t1053 = t386 * t386;
    let t1054 = 1.0_f64 / t1053;
    let t1055 = t68 * t1054;
    let t1057 = t1011 * t1014;
    let t1058 = t1010 * t1057;
    (t1050, t1052, t1053, t1055, t1057, t1058)
}
