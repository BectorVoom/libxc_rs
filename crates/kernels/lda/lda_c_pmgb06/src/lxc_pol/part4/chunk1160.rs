//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1160/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1160<F: Float>(t132: F, t137: F, t2064: F, t4815: F, t4979: F, t802: F, t1887: F, t2015: F, t4966: F, t486: F, t6731: F, t1499: F, t2654: F, t6461: F, t1969: F, t5187: F) -> (F, F, F, F, F, F, F, F) {
    let t17503 = 2.0 / 15.0 * t132 * t137 * t4815 * t2064;
    let t17505 = t802 * t4979 / 15.0;
    let t17506 = t1887 * t2015;
    let t17507 = 4.0 / 45.0 * t17506;
    let t17509 = t802 * t4966 / 15.0;
    let t17511 = 2.0 / 15.0 * t486 * t6731;
    let t17513 = t1499 * t2654 / 15.0;
    let t17515 = 2.0 / 15.0 * t486 * t6461;
    let t17517 = 4.0 / 15.0 * t5187 * t1969;
    (t17503, t17505, t17507, t17509, t17511, t17513, t17515, t17517)
}
