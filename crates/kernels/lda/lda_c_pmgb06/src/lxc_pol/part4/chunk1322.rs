//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1322/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1322<F: Float>(t17372: F, t1548: F, t2592: F, t1447: F, t6770: F, t17160: F, t1915: F, t493: F, t17296: F, t17300: F, t17304: F, t17305: F, t17306: F, t17307: F, t17308: F, t17367: F, t17369: F, t17371: F) -> (F, F, F, F, F) {
    let t17373 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t17372;
    let t17374 = t2592 * t1548;
    let t17375 = t17374 / F::cast_from(135.0_f64);
    let t17376 = t1447 * t6770;
    let t17377 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t17376;
    let t17380 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t1915 * t17160;
    let t17381 = -t17296 - t17300 - t17304 - t17305 - t17306 - t17307 - t17308 + t17367 + t17369 + t17371 - t17373 - t17375 - t17377 - t17380;
    (t17373, t17375, t17377, t17380, t17381)
}
