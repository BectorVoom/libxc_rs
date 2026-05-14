//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 894/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk894<F: Float>(t7190: F, t7210: F, t138: F, t1706: F, t1711: F, t1861: F, t1864: F, t1878: F, t2634: F, t2642: F, t3332: F, t3339: F, t444: F, t450: F, t5618: F, t5621: F, t7166: F, t7168: F, t7178: F, t7181: F, t7185: F, t774: F) -> (F, F) {
    let t7211 = t7190 + t7210;
    let t7213 = t7166 * t138 - t1706 * t2642 + 4.0 * t1711 * t7181 + 2.0 * t1711 * t7185 - 2.0 * t1861 * t1878 + 4.0 * t5621 * t1864 + 2.0 * t3332 * t2634 - 6.0 * t3339 * t7178 - t444 * t7211 - t7168 * t450 - 2.0 * t5618 * t774;
    (t7211, t7213)
}
