//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 844/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk844(t2253: f64, t656: f64, t2256: f64, t1410: f64, t851: f64, t5333: f64, t5336: f64, t5338: f64, t5341: f64, t5344: f64, t5346: f64, t5348: f64, t5350: f64, t5352: f64, t5354: f64, t5358: f64, t5362: f64, t5365: f64, t5369: f64) -> f64 {
    let t5871 = 4.0_f64 / 9.0_f64 * t2253 * t656;
    let t5872 = t2256 * t656;
    let t5874 = t851 * t1410;
    let t5876 = t5333 + t5336 + t5871 + 4.0_f64 / 9.0_f64 * t5872 - 2.0_f64 / 27.0_f64 * t5874 + t5338 - t5341 - t5344 + t5346 + t5348 + t5350 + t5352 - t5354 - t5358 + t5362 - t5365 - t5369;
    t5876
}
