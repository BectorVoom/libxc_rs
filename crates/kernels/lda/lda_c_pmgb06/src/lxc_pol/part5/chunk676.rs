//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 676/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk676<F: Float>(t5: F, t1874: F, t802: F, t1: F, t760: F, t2381: F, t332: F, t395: F, t5961: F, t44: F, t131: F, t155: F, t2592: F, t460: F, t1928: F, t2029: F, t4111: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t6694 = t802 * t1874 / 15.0;
    let t6695 = t760 * t1;
    let t6698 = t332 * t2381;
    let t6703 = piecewise3(t6, 0.0, 8.0 * t6695 * t395 + 2.0 * t5 * t5961 + 2.0 * t6698);
    let t6704 = t6703 * t44;
    let t6705 = t6704 * t131;
    let t6707 = t6705 * t155 / 30.0;
    let t6709 = t2592 * t460 / 30.0;
    let t6710 = t802 * t1928;
    let t6711 = 2.0 / 45.0 * t6710;
    let t6715 = 2e-21 * t2029 * t4111;
    (t6694, t6698, t6704, t6705, t6707, t6709, t6710, t6711, t6715)
}
