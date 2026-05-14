//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1050/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1050<F: Float>(t242: F, t4422: F, t4437: F, t5446: F, t646: F, t1426: F, t1901: F, t1896: F, t1: F, t3921: F, t5470: F, t2260: F, t3936: F, t5788: F, t656: F, t1410: F, t2253: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14960 = t4422 * t242;
    let t14965 = t4437 * t242;
    let t14978 = t5446 * t646;
    let t14980 = t1901 * t1426;
    let t14992 = t1896 * t646;
    let t15015 = t5470 * t1 * t3921;
    let t15060 = t2260 * t3936;
    let t15062 = t5788 * t656;
    let t15107 = t2253 * t1410;
    (t14960, t14965, t14978, t14980, t14992, t15015, t15060, t15062, t15107)
}
