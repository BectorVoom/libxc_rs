//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 648/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk648<F: Float>(t1798: F, t81: F, t199: F, t122: F, t1669: F, t886: F, t107: F, t1180: F, t902: F, t1795: F, t566: F, t1329: F, t868: F) -> (F, F, F, F, F, F) {
    let t5522 = t81 * t1798;
    let t5524 = F::cast_from(0.1675256410710088_f64) * t5522 * t199;
    let t5526 = t122 * t1669 * t886;
    let t5529 = t107 * t1180 * t902;
    let t5542 = F::cast_from(0.1675256410710088_f64) * t1795 * t566;
    let t5551 = F::cast_from(0.1675256410710088_f64) * t1329 * t868;
    (t5522, t5524, t5526, t5529, t5542, t5551)
}
