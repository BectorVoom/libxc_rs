//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1022<F: Float>(t3055: F, t802: F, t1392: F, t1887: F, t3068: F, t13995: F, t13997: F, t13999: F, t14002: F, t14005: F, t14007: F, t14010: F, t14012: F, t14014: F, t161: F, t489: F, t4940: F) -> (F, F, F, F, F) {
    let t14015 = t802 * t3055;
    let t14016 = t14015 / 45.0;
    let t14017 = t1887 * t1392;
    let t14018 = 2.0 / 15.0 * t14017;
    let t14019 = t802 * t3068;
    let t14020 = 2.0 / 15.0 * t14019;
    let t14021 = -t13995 - t13997 - t13999 - t14002 - t14005 - t14007 - t14010 - t14012 - t14014 + t14016 - t14018 - t14020;
    let t14024 = t161 * t489 * t4940;
    (t14016, t14018, t14020, t14021, t14024)
}
