//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1094/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1094<F: Float>(t132: F, t2851: F, t823: F, t1512: F, t2015: F, t432: F, t5302: F, t495: F, t5415: F, t224: F, t5431: F, t1423: F, t4609: F) -> (F, F, F, F, F, F) {
    let t13090 = t132 * t2851 * t823;
    let t13092 = t1512 * t2015;
    let t13094 = t432 * t5302;
    let t13100 = t495 * t5415;
    let t13104 = t5431 * t224;
    let t13117 = t1423 * t4609;
    (t13090, t13092, t13094, t13100, t13104, t13117)
}
