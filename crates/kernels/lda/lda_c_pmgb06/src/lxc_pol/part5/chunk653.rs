//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 653/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk653<F: Float>(t1409: F, t794: F, t188: F, t1798: F, t539: F, t856: F, t97: F, t1377: F, t2342: F, t27: F, t545: F, t2345: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5632 = t794 * t1409;
    let t5633 = t5632 * t188;
    let t5638 = t1798 * t539;
    let t5640 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5638 * t188;
    let t5649 = t856 * t97;
    let t5650 = t5649 * t1377;
    let t5652 = t2342 * t27;
    let t5654 = F::cast_from(0.21642082724729686_f64) * t5652 * t545;
    let t5655 = t2345 * t27;
    (t5632, t5633, t5638, t5640, t5649, t5650, t5652, t5654, t5655)
}
