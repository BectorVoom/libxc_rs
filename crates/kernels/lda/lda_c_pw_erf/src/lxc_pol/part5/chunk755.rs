//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 755/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk755<F: Float>(t4730: F, t6683: F, t6686: F, t6690: F, t6697: F, t6700: F, t6703: F, t6706: F, t6708: F, t2468: F, t822: F, t6193: F, t833: F, t1466: F, t571: F, t7007: F, t826: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7545 = 4.0 / 45.0 * t4730;
    let t7547 = 16.0 / 45.0 * t6683;
    let t7548 = 16.0 / 45.0 * t6686;
    let t7549 = 16.0 / 15.0 * t6690;
    let t7550 = 8.0 / 45.0 * t6697;
    let t7551 = 8.0 / 27.0 * t6700;
    let t7552 = 8.0 / 45.0 * t6703;
    let t7553 = 8.0 / 27.0 * t6706;
    let t7554 = 16.0 / 45.0 * t6708;
    let t7556 = 4.0 / 5.0 * t822 * t2468;
    let t7557 = t6193 * t833;
    let t7558 = t1466 * t7557;
    let t7560 = 4.0 / 5.0 * t571 * t7558;
    let t7562 = 8.0 / 15.0 * t7007 * t826;
    (t7545, t7547, t7548, t7549, t7550, t7551, t7552, t7553, t7554, t7556, t7557, t7558, t7560, t7562)
}
