//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1005/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1005<F: Float>(t110: F, t360: F, t7035: F, t7027: F, t365: F, t5772: F, t6989: F, t18588: F, t5770: F, t1271: F, t2712: F, t955: F) -> (F, F, F, F, F) {
    let t18622 = t360 * t110 * t7035;
    let t18625 = t360 * t110 * t7027;
    let t18628 = t365 * t6989 * t5772;
    let t18630 = t5770 * t18588;
    let t18718 = t1271 * t2712 * t955;
    (t18622, t18625, t18628, t18630, t18718)
}
