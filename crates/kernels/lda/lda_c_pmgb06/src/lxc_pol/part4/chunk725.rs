//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 725/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk725<F: Float>(t1454: F, t1988: F, t1461: F, t842: F, t1466: F, t1447: F, t1995: F, t1980: F, t485: F) -> (F, F, F, F, F) {
    let t4585 = t1988 * t1454;
    let t4588 = t1461 * t842;
    let t4589 = t4588 * t1466;
    let t4593 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1447 * t1995;
    let t4602 = t485 * t1980;
    (t4585, t4588, t4589, t4593, t4602)
}
