//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 983/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk983<F: Float>(t1055: F, t4393: F, t1051: F, t344: F, t5685: F, t339: F, t4384: F, t1: F, t1750: F, t1755: F, t2316: F, t4299: F, t4415: F, t2849: F, t749: F, t3160: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11371 = t4393 * t1055;
    let t11373 = t4393 * t1051;
    let t11387 = t344 * t5685;
    let t11389 = t339 * t4384;
    let t11391 = t344 * t4384;
    let t11397 = t2316 * t1750 * t1 * t1755;
    let t11399 = t4415 * t4299;
    let t11401 = t2849 * t749;
    let t11403 = t3160 * t749;
    (t11371, t11373, t11387, t11389, t11391, t11397, t11399, t11401, t11403)
}
