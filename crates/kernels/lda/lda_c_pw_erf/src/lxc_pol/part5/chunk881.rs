//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 881/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk881<F: Float>(t242: F, t4422: F, t4437: F, t5446: F, t646: F, t1426: F, t1901: F, t1896: F, t1: F, t3921: F, t5470: F, t2260: F, t3936: F, t1410: F, t2253: F, t2256: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14960 = t4422 * t242;
    let t14961 = 0.5025769232130264 * t14960;
    let t14965 = t4437 * t242;
    let t14978 = t5446 * t646;
    let t14979 = 0.09973633333333333 * t14978;
    let t14980 = t1901 * t1426;
    let t14992 = t1896 * t646;
    let t15015 = t5470 * t1 * t3921;
    let t15060 = t2260 * t3936;
    let t15107 = t2253 * t1410;
    let t15108 = 2.0 / 9.0 * t15107;
    let t15109 = t2256 * t1410;
    (t14961, t14965, t14979, t14980, t14992, t15015, t15060, t15108, t15109)
}
