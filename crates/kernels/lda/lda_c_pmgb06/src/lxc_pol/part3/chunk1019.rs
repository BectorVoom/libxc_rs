//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1019/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1019<F: Float>(t1902: F, t3177: F, t1420: F, t5254: F, t5257: F, t5261: F, t12106: F, t12108: F, t12110: F, t12113: F, t12114: F, t12115: F, t12116: F, t12117: F) -> (F, F, F, F, F) {
    let t12119 = t3177 * t1902 / F::new(9.0);
    let t12121 = F::new(2.0) / F::new(9.0) * t1420 * t5254;
    let t12123 = t1420 * t5257 / F::new(9.0);
    let t12125 = F::new(8.0) / F::new(27.0) * t1420 * t5261;
    let t12126 = t12106 + t12108 - t12110 + t12113 - t12114 - t12115 - t12116 - t12117 + t12119 + t12121 + t12123 + t12125;
    (t12119, t12121, t12123, t12125, t12126)
}
