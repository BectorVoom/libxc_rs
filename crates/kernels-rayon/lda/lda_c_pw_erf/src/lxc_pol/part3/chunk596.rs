//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 596/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk596(t101: f64, t3365: f64, t153: f64, t274: f64, t2869: f64, t1089: f64, t474: f64, t1125: f64, t678: f64, t1298: f64, t1386: f64, t1394: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3366 = t101 * t3365;
    let t3373 = 4.429070076315393_f64 * t153 * t2869 * t274;
    let t3375 = t153 * t474 * t1089;
    let t3378 = t153 * t1125 * t678;
    let t3380 = t1298 * t1386;
    let t3381 = 16.0_f64 / 15.0_f64 * t3380;
    let t3383 = 4.0_f64 / 5.0_f64 * t511 * t1394;
    (t3366, t3373, t3375, t3378, t3380, t3381, t3383)
}
