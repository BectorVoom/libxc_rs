//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 992/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk992<F: Float>(t4612: F, t5168: F, t2010: F, t2011: F, t3216: F, t1447: F, t5313: F, t4585: F, t13477: F, t13480: F, t13482: F, t13486: F, t13489: F, t13492: F, t13496: F, t4589: F) -> (F, F, F, F, F, F) {
    let t13498 = 4.0 / 15.0 * t5168 * t4612;
    let t13501 = 2.0 / 15.0 * t2010 * t3216 * t2011;
    let t13502 = t1447 * t5313;
    let t13503 = 4.0 / 45.0 * t13502;
    let t13504 = t1447 * t4585;
    let t13505 = 2.0 / 45.0 * t13504;
    let t13506 = t13477 + t13480 + t13482 + t13486 + t13489 + t13492 + t13496 + t13498 + t13501 + t13503 + t13505;
    let t13507 = t1447 * t4589;
    (t13498, t13501, t13503, t13505, t13506, t13507)
}
