//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 878/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk878<F: Float>(t973: F, t990: F, t983: F, t1011: F, t1028: F, t1030: F, t174: F, t2993: F, t2998: F, t3085: F, t325: F, t365: F, t370: F, t371: F, t386: F, t4606: F, t473: F, t5021: F, t62: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F, t8171: F, t8188: F, t8221: F, t8224: F, t8238: F, t8244: F, t8260: F, t8428: F, t8561: F, t971: F, t972: F, t974: F, t984: F, t988: F, t991: F) -> (F, F, F) {
    let t8564 = t973 * t973;
    let t8565 = t990 * t990;
    let t8586 = t983 * t983;
    let t8600 = F::new(1.0) * t365 * (-F::cast_from(3.9219166666666667_f64) * t8141 + F::new(37.6504) * t8143 - F::cast_from(13.944592592592592_f64) * t8146 + F::cast_from(12.201518518518519_f64) * t8149 + F::cast_from(5.356037037037037_f64) * t4606 + F::cast_from(0.14025833333333335_f64) * t8155 - F::cast_from(2.2441333333333335_f64) * t8157 + F::cast_from(2.4934814814814814_f64) * t8159 + F::cast_from(2.1817962962962962_f64) * t8161 + F::cast_from(1.6979925925925925_f64) * t5021) * t371 + F::cast_from(199659.08856856835_f64) * t62 / t8561 * t8564 / t8565 - F::cast_from(3.5089340384731225_f64) * t1011 * t8428 * t386 + t8188 + t8221 - t8224 - t8238 + t8244 - t8260 + F::new(0.41096) * t325 * t971 * t370 * t984 - F::cast_from(1.9263778438055648_f64) * t325 * t2998 + F::cast_from(0.1301229705933783_f64) * t325 * t2993 - F::cast_from(6.609199099388871_f64) * t325 * t988 * t983 * t991 * t370 - F::new(6.0) * t972 * t8586 * t371 - F::cast_from(14.03573615389249_f64) * t3085 * t8171 * t386 + F::cast_from(51.94726769812759_f64) * t1028 * t8428 * t1030 - F::cast_from(0.27397333333333335_f64) * t174 * t473 * t971 * t974;
    (t8564, t8586, t8600)
}
