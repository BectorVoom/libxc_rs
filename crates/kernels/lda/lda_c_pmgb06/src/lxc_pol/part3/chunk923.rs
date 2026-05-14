//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 923/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk923<F: Float>(t12431: F, t131: F, t155: F, t44: F, t460: F, t4754: F, t432: F, t4682: F, t1491: F, t1848: F, t12304: F, t12307: F, t12308: F, t12311: F, t12313: F, t12315: F, t12415: F, t12417: F) -> (F, F, F, F, F) {
    let t12435 = t12431 * t44 * t131 * t155 / 30.0;
    let t12437 = t4754 * t460 / 10.0;
    let t12439 = t432 * t4682 / 10.0;
    let t12441 = t1848 * t1491 / 10.0;
    let t12442 = 2.0 / 3.0 * t12304 + t12307 + 2.0 / 3.0 * t12308 + t12311 + t12313 + t12315 + t12415 + t12417 + t12435 + t12437 + t12439 + t12441;
    (t12435, t12437, t12439, t12441, t12442)
}
