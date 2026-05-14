//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 809/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk809<F: Float>(t987: F, t973: F, t990: F, t983: F, t1011: F, t1028: F, t1030: F, t174: F, t2993: F, t2998: F, t3085: F, t325: F, t365: F, t370: F, t371: F, t386: F, t4606: F, t473: F, t5021: F, t62: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F, t8171: F, t8188: F, t8221: F, t8224: F, t8238: F, t8244: F, t8260: F, t8428: F, t971: F, t972: F, t974: F, t984: F, t988: F, t991: F) -> (F, F, F) {
    let t8561 = t987 * t987;
    let t8564 = t973 * t973;
    let t8565 = t990 * t990;
    let t8586 = t983 * t983;
    let t8600 = 1.0 * t365 * (-3.9219166666666667 * t8141 + 37.6504 * t8143 - 13.944592592592592 * t8146 + 12.201518518518519 * t8149 + 5.356037037037037 * t4606 + 0.14025833333333335 * t8155 - 2.2441333333333335 * t8157 + 2.4934814814814814 * t8159 + 2.1817962962962962 * t8161 + 1.6979925925925925 * t5021) * t371 + 199659.08856856835 * t62 / t8561 * t8564 / t8565 - 3.5089340384731225 * t1011 * t8428 * t386 + t8188 + t8221 - t8224 - t8238 + t8244 - t8260 + 0.41096 * t325 * t971 * t370 * t984 - 1.9263778438055648 * t325 * t2998 + 0.1301229705933783 * t325 * t2993 - 6.609199099388871 * t325 * t988 * t983 * t991 * t370 - 6.0 * t972 * t8586 * t371 - 14.03573615389249 * t3085 * t8171 * t386 + 51.94726769812759 * t1028 * t8428 * t1030 - 0.27397333333333335 * t174 * t473 * t971 * t974;
    (t8564, t8586, t8600)
}
