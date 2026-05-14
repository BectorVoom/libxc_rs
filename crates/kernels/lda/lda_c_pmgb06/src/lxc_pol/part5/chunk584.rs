//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 584/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk584<F: Float>(t1122: F, t4549: F, t2148: F, t980: F, t968: F, t2142: F, t273: F, t698: F, t959: F, t3941: F, t3945: F, t3948: F, t3955: F, t2164: F, t395: F, t1461: F, t842: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4550 = t4549 * t1122;
    let t4552 = t2148 * t980;
    let t4554 = t2148 * t968;
    let t4556 = t2142 * t273;
    let t4558 = 1.1696447245269292 * t4556 * t698;
    let t4559 = t2148 * t959;
    let t4568 = 12.0 * t3941;
    let t4569 = 48.0 * t3945;
    let t4570 = 80.0 * t3948;
    let t4571 = 32.0 * t3955;
    let t4579 = 0.2133002709687175 * t395 * t2164;
    let t4588 = t1461 * t842;
    (t4550, t4552, t4554, t4556, t4558, t4559, t4568, t4569, t4570, t4571, t4579, t4588)
}
