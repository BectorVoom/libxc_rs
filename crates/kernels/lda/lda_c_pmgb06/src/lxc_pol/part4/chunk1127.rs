//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1127/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1127<F: Float>(t4044: F, t6007: F, t769: F, t1289: F, t4232: F, t10577: F, t4354: F, t2209: F, t384: F, t123: F, t317: F, t4575: F, t740: F) -> (F, F, F, F, F) {
    let t14587 = t6007 * t769 * t4044;
    let t14593 = t4232 * t769 * t1289;
    let t14596 = t10577 * t4354;
    let t14617 = t384 * t2209;
    let t14623 = t123 * t740 * t4575 * t317;
    (t14587, t14593, t14596, t14617, t14623)
}
