//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1104/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1104<F: Float>(t3247: F, t842: F, t1447: F, t5313: F, t4585: F, t4589: F, t1995: F, t3226: F, t146: F, t4989: F, t9712: F, t2060: F, t819: F) -> (F, F, F, F, F, F, F) {
    let t13483 = t3247 * t842;
    let t13502 = t1447 * t5313;
    let t13504 = t1447 * t4585;
    let t13507 = t1447 * t4589;
    let t13515 = t3226 * t1995;
    let t13532 = t146 * t9712 * t4989;
    let t13558 = t2060 * t819;
    (t13483, t13502, t13504, t13507, t13515, t13532, t13558)
}
