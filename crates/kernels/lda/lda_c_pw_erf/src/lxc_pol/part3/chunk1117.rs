//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1117/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1117<F: Float>(t1410: F, t2253: F, t2256: F, t3990: F, t851: F, t13925: F, t13927: F, t13930: F, t13933: F, t13937: F, t13939: F, t13941: F, t13945: F, t13952: F, t13956: F, t13958: F, t13961: F, t13965: F, t13969: F, t13972: F, t13974: F, t13976: F, t13978: F, t13979: F, t13980: F, t13981: F, t13983: F, t13984: F) -> (F, F) {
    let t15107 = t2253 * t1410;
    let t15108 = 2.0 / 9.0 * t15107;
    let t15109 = t2256 * t1410;
    let t15111 = t851 * t3990;
    let t15113 = -t13925 - t13927 - t15108 - 2.0 / 9.0 * t15109 + 8.0 / 81.0 * t15111 - t13930 + t13933 + t13937 + t13939 - t13941 - t13945 - t13952 + t13956;
    let t15114 = -t13958 + t13961 + t13965 - t13969 + t13972 - t13974 + t13976 + t13978 - t13979 - t13980 - t13981 + t13983 + t13984;
    (t15113, t15114)
}
