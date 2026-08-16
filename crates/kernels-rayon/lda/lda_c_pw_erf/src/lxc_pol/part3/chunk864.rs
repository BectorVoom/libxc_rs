//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 864/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk864(t1084: f64, t1125: f64, t402: f64, t156: f64, t2942: f64, t2948: f64, t2704: f64, t2707: f64, t1085: f64, t4: f64, t960: f64, t2737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8271 = 0.06747116993730726_f64 * t1084 * t1125 * t402;
    let t8274 = 0.1301229705933783_f64 * t1084 * t156 * t2942;
    let t8277 = 3.8527556876111295_f64 * t1084 * t156 * t2948;
    let t8278 = t2704 * t2707;
    let t8281 = t960 * t4 * t1085;
    let t8285 = 0.021687161765563047_f64 * t1084 * t156 * t2737;
    (t8271, t8274, t8277, t8278, t8281, t8285)
}
