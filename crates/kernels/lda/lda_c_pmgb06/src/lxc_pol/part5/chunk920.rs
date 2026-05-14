//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 920/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk920<F: Float>(t337: F, t7295: F, t9509: F, t36: F, t9507: F, t1: F, t6507: F, t1830: F, t2909: F, t7594: F, t1476: F, t6502: F, t18436: F, t8131: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19377 = t9509 * t7295 * t337;
    let t19379 = t36 * t9507 * t19377;
    let t19381 = t6507 * t1;
    let t19383 = t1830 * t2909 * t19381;
    let t19385 = t7594 * t337;
    let t19387 = t36 * t1476 * t19385;
    let t19389 = t6502 * t1;
    let t19391 = t1830 * t1476 * t19389;
    let t19395 = -t18436 - 24.0 * t8131;
    (t19377, t19379, t19381, t19383, t19385, t19387, t19389, t19391, t19395)
}
