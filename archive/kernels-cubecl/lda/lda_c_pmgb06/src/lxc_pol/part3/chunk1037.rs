//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1037/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1037<F: Float>(t1869: F, t8337: F, t1525: F, t1830: F, t3103: F, t2969: F, t453: F, t810: F, t3010: F, t4644: F, t36: F, t1069: F, t4654: F) -> (F, F, F, F, F, F, F) {
    let t12329 = t8337 * t1869;
    let t12332 = t1830 * t1525 * t3103;
    let t12335 = t1830 * t453 * t2969;
    let t12337 = t1830 * t810;
    let t12339 = t4644 * t3010;
    let t12341 = t36 * t1525 * t12339;
    let t12343 = t4654 * t1069;
    (t12329, t12332, t12335, t12337, t12339, t12341, t12343)
}
