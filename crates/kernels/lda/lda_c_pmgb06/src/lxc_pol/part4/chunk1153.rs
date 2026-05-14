//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1153/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1153<F: Float>(t1499: F, t2555: F, t486: F, t6833: F, t5051: F, t802: F, t1548: F, t2592: F, t1447: F, t6770: F, t17160: F, t1915: F, t493: F, t17296: F, t17300: F, t17304: F, t17305: F, t17306: F, t17307: F, t17308: F, t17367: F) -> (F, F, F, F, F, F, F) {
    let t17369 = t1499 * t2555 / 30.0;
    let t17371 = t486 * t6833 / 15.0;
    let t17372 = t802 * t5051;
    let t17373 = 2.0 / 135.0 * t17372;
    let t17374 = t2592 * t1548;
    let t17375 = t17374 / 135.0;
    let t17376 = t1447 * t6770;
    let t17377 = 4.0 / 81.0 * t17376;
    let t17380 = 2.0 / 45.0 * t493 * t1915 * t17160;
    let t17381 = -t17296 - t17300 - t17304 - t17305 - t17306 - t17307 - t17308 + t17367 + t17369 + t17371 - t17373 - t17375 - t17377 - t17380;
    (t17369, t17371, t17373, t17375, t17377, t17380, t17381)
}
