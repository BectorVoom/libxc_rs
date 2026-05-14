//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1019/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1019<F: Float>(t14956: F, t14966: F, t14981: F, t14993: F, t15006: F, t15019: F, t15038: F, t15056: F, t11639: F, t123: F, t1312: F, t1316: F, t14561: F, t14875: F, t2258: F, t24: F, t315: F, t317: F, t346: F, t388: F, t4013: F, t4044: F, t4053: F, t4398: F, t4414: F, t5721: F, t5903: F, t6009: F, t6018: F, t6021: F, t6028: F, t7099: F, t7102: F, t787: F, t8070: F, t8074: F, t8077: F, t8081: F, t8087: F, t8091: F) -> (F, F) {
    let t15059 = t14956 + t14966 + t14981 + t14993 + t15006 + t15019 + t15038 + t15056;
    let t15081 = -t346 * t6021 * t4053 + 24.0 * t6018 * t14561 + 3.0 * t1316 * t1312 * t7102 + 6.0 * t1316 * t388 * t14875 + 2.0 * t346 * t5903 * t787 * t4044 + 12.0 * t1316 * t2258 * t4414 + 0.020267214298646783 * t123 * t315 * t15059 * t317 + 12.0 * t1316 * t2258 * t5721 - 2.0 * t346 * t4398 * t6028 - 2.0 * t346 * t6021 * t4013 + 4.0 * t24 * t11639 * t6009 - 2.0 * t346 * t4398 * t7099 - 0.00010931146159029059 * t8070 - t8074 + 0.0003279343847708718 * t8077 + t8081 - t8087 - t8091;
    (t15059, t15081)
}
