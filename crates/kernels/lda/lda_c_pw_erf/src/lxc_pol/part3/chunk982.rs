//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 982/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk982<F: Float>(t344: F, t5685: F, t339: F, t4384: F, t1: F, t1750: F, t1755: F, t2316: F, t4299: F, t4415: F, t2849: F, t749: F) -> (F, F, F, F, F, F) {
    let t11387 = t344 * t5685;
    let t11388 = F::new(24.0) * t11387;
    let t11389 = t339 * t4384;
    let t11390 = F::new(12.0) * t11389;
    let t11391 = t344 * t4384;
    let t11392 = F::new(12.0) * t11391;
    let t11397 = t2316 * t1750 * t1 * t1755;
    let t11398 = F::new(1.898172889849454) * t11397;
    let t11399 = t4415 * t4299;
    let t11401 = t2849 * t749;
    (t11388, t11390, t11392, t11398, t11399, t11401)
}
