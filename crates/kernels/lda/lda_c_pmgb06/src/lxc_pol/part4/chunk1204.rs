//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1204/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1204<F: Float>(t15925: F, t15927: F, t15930: F, t15934: F, t15939: F, t15942: F, t15944: F, t15946: F, t15950: F, t15953: F, t15955: F, t15957: F, t15959: F, t15962: F, t15965: F, t15968: F, t15971: F, t15973: F, t15975: F, t15978: F, t15980: F, t15982: F, t15983: F, t15984: F, t15987: F, t15990: F, t15992: F, t15996: F, t16000: F, t16002: F) -> (F, F) {
    let t18188 = -t15925 - t15927 - t15930 - t15934 - t15939 + t15942 - t15944 - t15946 - t15950 - t15953 - t15955 - t15957 - t15959 + t15962 - t15965;
    let t18189 = -t15968 + t15971 - t15973 + t15975 + t15978 + t15980 - t15982 - t15983 - t15984 - t15987 - t15990 - t15992 - t15996 - t16000 + t16002;
    (t18188, t18189)
}
