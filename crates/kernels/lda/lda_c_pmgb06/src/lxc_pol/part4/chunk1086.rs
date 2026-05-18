//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1086/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1086<F: Float>(t1447: F, t4757: F, t1989: F, t3223: F, t1980: F, t883: F, t4713: F, t607: F, t1710: F, t1959: F, t1423: F, t4767: F) -> (F, F, F, F, F, F) {
    let t12644 = t1447 * t4757;
    let t12649 = t3223 * t1989;
    let t12657 = t883 * t1980;
    let t12659 = t4713 * t607;
    let t12661 = t1959 * t1710;
    let t12677 = t1423 * t4767;
    (t12644, t12649, t12657, t12659, t12661, t12677)
}
