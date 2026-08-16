//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 758/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk758(t2168: f64, t3794: f64, t1472: f64, t2140: f64, t1446: f64, t2188: f64, t4804: f64, t1443: f64, t4738: f64, t2183: f64, t2193: f64, t4753: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4915 = 8.0_f64 / 15.0_f64 * t3794 * t2168;
    let t4917 = 16.0_f64 / 135.0_f64 * t1472 * t2140;
    let t4919 = 8.0_f64 / 15.0_f64 * t1446 * t2188;
    let t4921 = 8.0_f64 / 15.0_f64 * t4804 * t2168;
    let t4923 = 8.0_f64 / 15.0_f64 * t4738 * t1443;
    let t4925 = 8.0_f64 / 15.0_f64 * t3794 * t2183;
    let t4927 = 8.0_f64 / 15.0_f64 * t4753 * t2193;
    (t4915, t4917, t4919, t4921, t4923, t4925, t4927)
}
