//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 977/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk977(t8208: f64, t8212: f64, t8216: f64, t339: f64, t4405: f64, t11333: f64, t11334: f64, t11336: f64, t11338: f64, t11340: f64, t11341: f64, t11342: f64, t11343: f64, t11344: f64, t8202: f64, t8221: f64, t8224: f64, t8238: f64, t8244: f64, t8248: f64) -> (f64, f64, f64, f64, f64) {
    let t11345 = 60.0_f64 * t8208;
    let t11346 = 360.0_f64 * t8212;
    let t11347 = 36.0_f64 * t8216;
    let t11348 = t339 * t4405;
    let t11349 = 12.0_f64 * t11348;
    let t11350 = -t11333 - t11334 - t11336 - t11338 + t11340 + t11341 + t11342 - t8202 + t11343 + t11344 + t11345 - t11346 - t11347 + t11349 - t8221 + t8224 + t8238 - t8244 - t8248;
    (t11345, t11346, t11347, t11349, t11350)
}
