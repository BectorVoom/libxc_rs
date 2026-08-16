//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 894/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk894<F: Float>(t3267: F, t8980: F, t1664: F, t299: F, t732: F, t1686: F, t1697: F, t19: F, t119: F, t1568: F, t473: F, t1691: F) -> (F, F, F, F, F) {
    let t8985 = t3267 * t8980;
    let t8990 = t732 * t299 * t1664;
    let t8991 = t1686 * t1697 * t19 * t8990;
    let t8994 = t119 * t473 * t1568;
    let t8995 = t1691 * t8994;
    (t8985, t8990, t8991, t8994, t8995)
}
