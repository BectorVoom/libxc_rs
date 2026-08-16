//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 658/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk658(t8469: f64, t935: f64, t2580: f64, t2508: f64, t2530: f64, t2958: f64, t10677: f64, t701: f64, t1901: f64, t7659: f64, t9014: f64, t3444: f64, t731: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10713 = t8469 * t935;
    let t10714 = t2580 * t10713;
    let t10716 = 0.15381052460284448567e-1_f64 * t2508 * t10714;
    let t10717 = t2958 * t2530;
    let t10718 = t2580 * t10717;
    let t10720 = 0.15381052460284448567e-1_f64 * t2508 * t10718;
    let t10721 = t10677 * t701;
    let t10722 = t1901 * t10721;
    let t10731 = t9014 * t7659;
    let t10733 = 0.92286314761706691403e-1_f64 * t2508 * t10731;
    let t10734 = t731 * t3444;
    (t10713, t10716, t10717, t10720, t10721, t10722, t10733, t10734)
}
