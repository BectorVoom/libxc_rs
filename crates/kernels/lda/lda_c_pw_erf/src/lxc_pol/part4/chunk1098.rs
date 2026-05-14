//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1098/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1098<F: Float>(t12019: F, t13035: F, t6767: F, t34: F, t4495: F, t12362: F, t4494: F, t2325: F, t348: F, t494: F, t12387: F, t3965: F, t15825: F, t739: F, t12475: F, t5141: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15996 = 32.0 / 81.0 * t12019;
    let t15998 = 32.0 / 27.0 * t13035 * t6767;
    let t15999 = t4495 * t34;
    let t16002 = 64.0 / 45.0 * t12362 * t4494 * t15999;
    let t16003 = t2325 * t348;
    let t16004 = t16003 * t494;
    let t16007 = 32.0 / 15.0 * t3965 * t12387 * t16004;
    let t16010 = 64.0 / 15.0 * t12362 * t12387 * t15825;
    let t16012 = t739 * t34 * t494;
    let t16015 = 128.0 / 45.0 * t12475 * t5141 * t16012;
    (t15996, t15998, t15999, t16002, t16003, t16004, t16007, t16010, t16012, t16015)
}
