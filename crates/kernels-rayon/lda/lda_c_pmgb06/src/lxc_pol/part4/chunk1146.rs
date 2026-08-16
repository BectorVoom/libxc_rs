//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1146/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1146(t14956: f64, t14966: f64, t14981: f64, t14993: f64, t15006: f64, t15019: f64, t15038: f64, t15056: f64, t11639: f64, t123: f64, t1312: f64, t1316: f64, t14561: f64, t14875: f64, t2258: f64, t24: f64, t315: f64, t317: f64, t346: f64, t388: f64, t4013: f64, t4044: f64, t4053: f64, t4398: f64, t4414: f64, t5721: f64, t5903: f64, t6009: f64, t6018: f64, t6021: f64, t6028: f64, t7099: f64, t7102: f64, t787: f64, t8070: f64, t8074: f64, t8077: f64, t8081: f64, t8087: f64, t8091: f64) -> (f64, f64) {
    let t15059 = t14956 + t14966 + t14981 + t14993 + t15006 + t15019 + t15038 + t15056;
    let t15081 = -t346 * t6021 * t4053 + 24.0_f64 * t6018 * t14561 + 3.0_f64 * t1316 * t1312 * t7102 + 6.0_f64 * t1316 * t388 * t14875 + 2.0_f64 * t346 * t5903 * t787 * t4044 + 12.0_f64 * t1316 * t2258 * t4414 + 0.020267214298646783_f64 * t123 * t315 * t15059 * t317 + 12.0_f64 * t1316 * t2258 * t5721 - 2.0_f64 * t346 * t4398 * t6028 - 2.0_f64 * t346 * t6021 * t4013 + 4.0_f64 * t24 * t11639 * t6009 - 2.0_f64 * t346 * t4398 * t7099 - 0.00010931146159029059_f64 * t8070 - t8074 + 0.0003279343847708718_f64 * t8077 + t8081 - t8087 - t8091;
    (t15059, t15081)
}
