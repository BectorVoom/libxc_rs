//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 361/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk361<F: Float>(t1529: F, t211: F, t1125: F, t153: F, t274: F, t474: F, t678: F, t450: F, t454: F, t142: F, t131: F) -> (F, F, F, F, F, F, F) {
    let t1531 = 4.0 / 135.0 * t211 * t1529;
    let t1540 = 1.328721022894618 * t153 * t1125 * t274;
    let t1542 = t153 * t474 * t678;
    let t1549 = t454 * t450;
    let t1550 = t1549 * t142;
    let t1552 = t131 * t131;
    let t1553 = 1.0 / t1552;
    (t1531, t1540, t1542, t1549, t1550, t1552, t1553)
}
