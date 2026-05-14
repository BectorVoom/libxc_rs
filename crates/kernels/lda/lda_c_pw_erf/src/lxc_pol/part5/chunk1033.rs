//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1033/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1033<F: Float>(t1390: F, t7792: F, t1440: F, t519: F, t542: F, t529: F, t1325: F, t494: F, t2171: F, t7004: F, t15685: F, t6981: F, t581: F, t7836: F, t1318: F, t1466: F, t593: F) -> (F, F, F, F, F) {
    let t21531 = t1390 * t7792;
    let t21535 = 4.0 / 15.0 * t519 * t1440 * t21531 * t542;
    let t21536 = t529 * t7792;
    let t21540 = 4.0 / 15.0 * t1325 * t1440 * t21536 * t494;
    let t21542 = 4.0 / 5.0 * t2171 * t7004;
    let t21544 = 4.0 / 5.0 * t15685 * t6981;
    let t21545 = t581 * t7836;
    let t21549 = 4.0 / 15.0 * t1318 * t1466 * t21545 * t593;
    (t21535, t21540, t21542, t21544, t21549)
}
