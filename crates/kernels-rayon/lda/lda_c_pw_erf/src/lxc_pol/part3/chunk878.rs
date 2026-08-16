//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 878/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk878(t973: f64, t990: f64, t983: f64, t1011: f64, t1028: f64, t1030: f64, t174: f64, t2993: f64, t2998: f64, t3085: f64, t325: f64, t365: f64, t370: f64, t371: f64, t386: f64, t4606: f64, t473: f64, t5021: f64, t62: f64, t8141: f64, t8143: f64, t8146: f64, t8149: f64, t8155: f64, t8157: f64, t8159: f64, t8161: f64, t8171: f64, t8188: f64, t8221: f64, t8224: f64, t8238: f64, t8244: f64, t8260: f64, t8428: f64, t8561: f64, t971: f64, t972: f64, t974: f64, t984: f64, t988: f64, t991: f64) -> (f64, f64, f64) {
    let t8564 = t973 * t973;
    let t8565 = t990 * t990;
    let t8586 = t983 * t983;
    let t8600 = 1.0_f64 * t365 * (-3.9219166666666667_f64 * t8141 + 37.6504_f64 * t8143 - 13.944592592592592_f64 * t8146 + 12.201518518518519_f64 * t8149 + 5.356037037037037_f64 * t4606 + 0.14025833333333335_f64 * t8155 - 2.2441333333333335_f64 * t8157 + 2.4934814814814814_f64 * t8159 + 2.1817962962962962_f64 * t8161 + 1.6979925925925925_f64 * t5021) * t371 + 199659.08856856835_f64 * t62 / t8561 * t8564 / t8565 - 3.5089340384731225_f64 * t1011 * t8428 * t386 + t8188 + t8221 - t8224 - t8238 + t8244 - t8260 + 0.41096_f64 * t325 * t971 * t370 * t984 - 1.9263778438055648_f64 * t325 * t2998 + 0.1301229705933783_f64 * t325 * t2993 - 6.609199099388871_f64 * t325 * t988 * t983 * t991 * t370 - 6.0_f64 * t972 * t8586 * t371 - 14.03573615389249_f64 * t3085 * t8171 * t386 + 51.94726769812759_f64 * t1028 * t8428 * t1030 - 0.27397333333333335_f64 * t174 * t473 * t971 * t974;
    (t8564, t8586, t8600)
}
