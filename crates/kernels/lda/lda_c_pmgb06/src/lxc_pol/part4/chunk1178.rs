//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1178/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1178<F: Float>(t1848: F, t1933: F, t1420: F, t6491: F, t12772: F, t1893: F, t439: F, t5290: F, t5482: F, t2010: F, t5294: F, t15468: F, t15469: F, t15470: F, t15471: F, t15473: F, t15474: F, t15475: F, t15476: F, t15480: F) -> (F, F, F, F, F, F) {
    let t15481 = t1848 * t1933;
    let t15482 = F::new(4.0) / F::new(45.0) * t15481;
    let t15484 = F::new(4.0) / F::new(45.0) * t1420 * t6491;
    let t15487 = F::new(4.0) / F::new(45.0) * t439 * t12772 * t1893;
    let t15490 = F::new(2.0) / F::new(45.0) * t439 * t5482 * t5290;
    let t15493 = F::new(8.0) / F::new(45.0) * t2010 * t5482 * t5294;
    let t15494 = -t15468 + t15469 - t15470 - t15471 - t15473 - t15474 - t15475 - t15476 - t15480 + t15482 - t15484 - t15487 - t15490 - t15493;
    (t15482, t15484, t15487, t15490, t15493, t15494)
}
