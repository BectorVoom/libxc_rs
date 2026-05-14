//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 658/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk658<F: Float>(t1730: F, t573: F, t580: F, t122: F, t1669: F, t610: F, t1735: F, t569: F, t107: F, t2786: F, t290: F, t1147: F, t31: F) -> (F, F, F, F, F, F) {
    let t4162 = t573 * t1730;
    let t4165 = 0.09973633333333333 * t580 * t1730;
    let t4174 = t122 * t1669 * t610;
    let t4177 = t122 * t569 * t1735;
    let t4181 = 4.429070076315393 * t107 * t2786 * t290;
    let t4182 = t31 * t1147;
    (t4162, t4165, t4174, t4177, t4181, t4182)
}
