//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 593/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk593<F: Float>(t3278: F, t3326: F, t1704: F, t443: F, t1710: F, t440: F, t442: F, t131: F, t1712: F, t450: F, t1724: F, t1125: F, t120: F, t133: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3327 = t3278 + t3326;
    let t3329 = t1704 * t443;
    let t3332 = t440 * t1710;
    let t3337 = t442 * t442;
    let t3338 = F::cast_from(1.0_f64) / t3337;
    let t3339 = t131 * t3338;
    let t3340 = t1712 * t450;
    let t3343 = t450 * t1724;
    let t3348 = F::cast_from(0.8940581481481481_f64) * t133 * t1125 * t120;
    (t3327, t3329, t3332, t3337, t3338, t3339, t3340, t3343, t3348)
}
