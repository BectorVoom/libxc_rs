//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1360/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1360<F: Float>(t17875: F, t1631: F, t2592: F, t14011: F, t14015: F, t14465: F, t14467: F, t14469: F, t14471: F, t17858: F, t17859: F, t17861: F, t17863: F, t17869: F, t17871: F, t17873: F) -> (F, F, F, F, F) {
    let t17876 = F::new(2.0) / F::new(45.0) * t17875;
    let t17878 = t2592 * t1631 / F::new(30.0);
    let t17879 = F::new(2.0) / F::new(45.0) * t14011;
    let t17880 = F::new(4.0) / F::new(135.0) * t14015;
    let t17881 = -t17858 + F::new(8.0) / F::new(3.0) * t17859 + F::new(8.0) / F::new(3.0) * t17861 + t17863 + F::new(8.0) * t14465 + F::cast_from(0.002206740740740741_f64) * t14467 + F::new(8.0) / F::new(3.0) * t14469 + F::new(32.0) / F::new(3.0) * t14471 - t17869 - t17871 + t17873 - t17876 - t17878 - t17879 + t17880;
    (t17876, t17878, t17879, t17880, t17881)
}
