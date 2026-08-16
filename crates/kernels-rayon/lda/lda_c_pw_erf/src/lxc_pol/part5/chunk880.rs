//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 880/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk880(t1077: f64, t1084: f64, t474: f64, t1051: f64, t2704: f64, t2710: f64, t1125: f64, t402: f64, t156: f64, t2942: f64, t2948: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8263 = 0.08674864706225219_f64 * t1084 * t474 * t1077;
    let t8266 = 0.043374323531126094_f64 * t1084 * t474 * t1051;
    let t8267 = t2704 * t2710;
    let t8271 = 0.06747116993730726_f64 * t1084 * t1125 * t402;
    let t8274 = 0.1301229705933783_f64 * t1084 * t156 * t2942;
    let t8277 = 3.8527556876111295_f64 * t1084 * t156 * t2948;
    let t8278 = t2704 * t2707;
    (t8263, t8266, t8267, t8271, t8274, t8277, t8278)
}
