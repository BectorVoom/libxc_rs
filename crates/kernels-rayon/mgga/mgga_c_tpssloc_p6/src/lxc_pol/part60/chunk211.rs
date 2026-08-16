//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 211/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk211(t368: f64, t372: f64, t364: f64, t354: f64, t270: f64, t283: f64, t61: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1039 = t368 * t372;
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = 1.0_f64 / t283 / t270;
    let t1044 = t61 * t1043;
    let t1052 = t382 * t225;
    let t1053 = t386 * t386;
    let t1054 = 1.0_f64 / t1053;
    let t1055 = t68 * t1054;
    let t1057 = t1011 * t1014;
    (t1040, t1041, t1043, t1044, t1052, t1053, t1055, t1057)
}
