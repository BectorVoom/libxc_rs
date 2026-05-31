//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1006/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1006<F: Float>(t9424: F, t443: F, t464: F, t2010: F, t442: F, t477: F, t1423: F, t5291: F, t1420: F, t5365: F, t10288: F, t439: F, t5364: F) -> (F, F, F, F, F) {
    let t11964 = F::cast_from(2e-21_f64) * t9424;
    let t11966 = t464 * t443;
    let t11970 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2010 * t442 * t11966 * t477;
    let t11971 = t1423 * t5291;
    let t11972 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t11971;
    let t11974 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t5365;
    let t11977 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t10288 * t5364;
    (t11964, t11970, t11972, t11974, t11977)
}
