//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 894/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk894<F: Float>(t9424: F, t443: F, t464: F, t2010: F, t442: F, t477: F, t1423: F, t5291: F, t1420: F, t5365: F, t10288: F, t439: F, t5364: F, t1602: F, t1831: F, t1981: F, t2871: F) -> (F, F, F, F, F, F) {
    let t11964 = 2e-21 * t9424;
    let t11966 = t464 * t443;
    let t11970 = 2.0 / 15.0 * t2010 * t442 * t11966 * t477;
    let t11971 = t1423 * t5291;
    let t11972 = 2.0 / 45.0 * t11971;
    let t11974 = 2.0 / 15.0 * t1420 * t5365;
    let t11977 = 2.0 / 15.0 * t439 * t10288 * t5364;
    let t11981 = 4.0 / 15.0 * t1981 * t2871 * t1831 * t1602;
    (t11964, t11970, t11972, t11974, t11977, t11981)
}
