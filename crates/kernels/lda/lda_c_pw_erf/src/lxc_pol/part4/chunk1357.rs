//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1357/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1357<F: Float>(t138: F, t14729: F, t1711: F, t1724: F, t1861: F, t1864: F, t19504: F, t19542: F, t19561: F, t19586: F, t19633: F, t19659: F, t19677: F, t19697: F, t2634: F, t2642: F, t3332: F, t3339: F, t450: F, t5621: F, t5633: F, t5636: F, t5667: F, t7178: F, t7181: F, t7185: F, t7211: F, t774: F, t9054: F, t9059: F) -> (F,) {
    let t19702 = 4.0 * t5621 * t5636 + 8.0 * t14729 * t1864 - 6.0 * t3339 * t2634 * t1724 + 8.0 * t5621 * t5633 + 4.0 * t1711 * t7211 * t450 + 4.0 * t3332 * t7185 + 4.0 * t1711 * t774 * t5667 + 2.0 * t1711 * t2642 * t1724 - 12.0 * t9059 * t7178 + 8.0 * t3332 * t7181 + 2.0 * t9054 * t2634 - 2.0 * t1861 * t5667 + (t19504 + t19542 + t19561 + t19586 + t19633 + t19659 + t19677 + t19697) * t138;
    (t19702,)
}
