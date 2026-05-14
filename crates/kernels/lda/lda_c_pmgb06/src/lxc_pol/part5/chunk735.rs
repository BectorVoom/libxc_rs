//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 735/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk735<F: Float>(t5: F, t6307: F, t6309: F, t6311: F, t6313: F, t6315: F, t6317: F, t6319: F, t6321: F, t6323: F, t6325: F, t4777: F, t2381: F, t760: F, t7290: F, t44: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t7447 = 4.0 / 45.0 * t6307;
    let t7448 = 4.0 / 45.0 * t6309;
    let t7449 = 2.0 / 45.0 * t6311;
    let t7450 = 2.0 / 27.0 * t6313;
    let t7451 = 4.0 / 45.0 * t6315;
    let t7452 = 4.0 / 45.0 * t6317;
    let t7453 = 4.0 / 45.0 * t6319;
    let t7454 = 4.0 / 45.0 * t6321;
    let t7455 = 2.0 / 45.0 * t6323;
    let t7456 = 2.0 / 27.0 * t6325;
    let t7457 = 2.0 / 135.0 * t4777;
    let t7458 = t760 * t2381;
    let t7463 = piecewise3(t6, 0.0, 2.0 * t5 * t7290 + 6.0 * t7458);
    let t7464 = t7463 * t44;
    (t7447, t7448, t7449, t7450, t7451, t7452, t7453, t7454, t7455, t7456, t7457, t7458, t7464)
}
