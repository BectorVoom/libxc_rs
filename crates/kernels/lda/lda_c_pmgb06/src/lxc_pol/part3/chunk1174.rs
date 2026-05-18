//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1174/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1174<F: Float>(t2956: F, t439: F, t5482: F, t3453: F, t831: F, t1499: F, t2095: F, t3055: F, t802: F, t1392: F, t1887: F, t3068: F) -> (F, F, F, F, F, F) {
    let t14010 = t439 * t5482 * t2956 / F::new(15.0);
    let t14011 = t831 * t3453;
    let t14012 = t14011 / F::new(15.0);
    let t14014 = t1499 * t2095 / F::new(10.0);
    let t14015 = t802 * t3055;
    let t14016 = t14015 / F::new(45.0);
    let t14017 = t1887 * t1392;
    let t14018 = F::new(2.0) / F::new(15.0) * t14017;
    let t14019 = t802 * t3068;
    (t14010, t14012, t14014, t14016, t14018, t14019)
}
