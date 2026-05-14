//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 979/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk979<F: Float>(t1179: F, t132: F, t441: F, t4829: F, t1554: F, t161: F, t2089: F, t2851: F, t823: F, t1512: F, t2015: F, t432: F, t5302: F, t495: F, t5415: F, t224: F, t5431: F) -> (F, F, F, F, F, F, F) {
    let t13079 = t132 * t1179 * t441 * t4829;
    let t13087 = t161 * t1554 * t2089;
    let t13090 = t132 * t2851 * t823;
    let t13092 = t1512 * t2015;
    let t13094 = t432 * t5302;
    let t13100 = t495 * t5415;
    let t13104 = t5431 * t224;
    (t13079, t13087, t13090, t13092, t13094, t13100, t13104)
}
