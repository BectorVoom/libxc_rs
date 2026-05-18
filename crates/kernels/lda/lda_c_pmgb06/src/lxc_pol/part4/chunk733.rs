//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 733/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk733<F: Float>(t12: F, t1: F, t15: F, t1080: F, t1083: F, t1949: F, t1952: F, t247: F, t395: F, t4382: F, t598: F, t765: F, t44: F, t4697: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t4700 = t15 * t1;
    let t4710 = piecewise3::<f64>(t13, F::new(0.0), F::new(80.0) / F::new(27.0) * t765 * t1080 - F::new(160.0) / F::new(9.0) * t4700 * t4382 + F::new(40.0) / F::new(9.0) * t1949 * t1083 - F::new(16.0) / F::new(3.0) * t598 * t395 + F::new(16.0) * t1952 * t247);
    let t4713 = (t4697 / F::new(2.0) + t4710 / F::new(2.0)) * t44;
    t4713
}
