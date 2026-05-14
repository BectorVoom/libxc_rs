//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 982/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk982<F: Float>(t1070: F, t1775: F, t1067: F, t1799: F, t339: F, t4405: F, t402: F, t4383: F, t75: F, t1034: F, t1798: F, t40: F, t3153: F, t748: F, t1765: F, t2987: F) -> (F, F, F, F, F, F, F) {
    let t11337 = t1070 * t1775;
    let t11339 = t1067 * t1799;
    let t11348 = t339 * t4405;
    let t11355 = t4383 * t75 * t402;
    let t11359 = t40 * t1798 * t1034;
    let t11362 = t40 * t748 * t3153;
    let t11369 = t1765 * t2987;
    (t11337, t11339, t11348, t11355, t11359, t11362, t11369)
}
