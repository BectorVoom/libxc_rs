//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1277/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1277<F: Float>(t495: F, t6831: F, t493: F, t499: F, t132: F, t1547: F, t2649: F, t13087: F, t12535: F, t13027: F, t15324: F, t3259: F, t5075: F) -> (F, F, F, F) {
    let t16794 = t495 * t6831;
    let t16797 = F::new(2.0) / F::new(45.0) * t493 * t16794 * t499;
    let t16799 = t132 * t1547 * t2649;
    let t16800 = t16799 / F::new(135.0);
    let t16801 = F::new(4.0) / F::new(135.0) * t13087;
    let t16806 = F::new(64.0) / F::new(81.0) * t5075 * t12535 * t3259 * t13027 * t15324;
    (t16797, t16800, t16801, t16806)
}
