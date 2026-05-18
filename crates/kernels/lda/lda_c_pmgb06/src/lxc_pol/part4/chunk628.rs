//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 628/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk628<F: Float>(t1640: F, t489: F, t161: F, t1489: F, t517: F, t1179: F, t139: F, t138: F, t163: F, t508: F, t947: F, t1478: F, t350: F) -> (F, F, F, F, F, F, F, F) {
    let t2880 = t489 * t1640;
    let t2881 = t161 * t2880;
    let t2885 = t1489 * t517;
    let t2897 = t1179 * t139;
    let t2899 = t138 * t2897 * t163;
    let t2900 = F::new(0.005877407407407408) * t2899;
    let t2901 = t947 * t508;
    let t2903 = t350 * t1478;
    (t2880, t2881, t2885, t2897, t2899, t2900, t2901, t2903)
}
