//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 977/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk977<F: Float>(t8208: F, t8212: F, t8216: F, t339: F, t4405: F, t11333: F, t11334: F, t11336: F, t11338: F, t11340: F, t11341: F, t11342: F, t11343: F, t11344: F, t8202: F, t8221: F, t8224: F, t8238: F, t8244: F, t8248: F) -> (F, F, F, F, F) {
    let t11345 = F::cast_from(60.0_f64) * t8208;
    let t11346 = F::cast_from(360.0_f64) * t8212;
    let t11347 = F::cast_from(36.0_f64) * t8216;
    let t11348 = t339 * t4405;
    let t11349 = F::cast_from(12.0_f64) * t11348;
    let t11350 = -t11333 - t11334 - t11336 - t11338 + t11340 + t11341 + t11342 - t8202 + t11343 + t11344 + t11345 - t11346 - t11347 + t11349 - t8221 + t8224 + t8238 - t8244 - t8248;
    (t11345, t11346, t11347, t11349, t11350)
}
