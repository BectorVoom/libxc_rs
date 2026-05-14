//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 664/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk664<F: Float>(t1919: F, t6512: F, t1981: F, t2599: F, t497: F, t337: F, t2871: F, t493: F, t6474: F, t6477: F, t6480: F, t6482: F, t6484: F, t6486: F, t6488: F, t6490: F, t6493: F, t6497: F, t6501: F, t6506: F, t6511: F) -> (F, F, F, F, F, F, F) {
    let t6513 = t1919 * t6512;
    let t6515 = 4.0 / 27.0 * t1981 * t6513;
    let t6516 = t2599 * t497;
    let t6517 = t6516 * t337;
    let t6518 = t2871 * t6517;
    let t6520 = 2.0 / 45.0 * t493 * t6518;
    let t6521 = -t6474 + t6477 + t6480 - t6482 + t6484 - t6486 - t6488 + t6490 - t6493 - t6497 + t6501 - t6506 + t6511 - t6515 + t6520;
    (t6513, t6515, t6516, t6517, t6518, t6520, t6521)
}
