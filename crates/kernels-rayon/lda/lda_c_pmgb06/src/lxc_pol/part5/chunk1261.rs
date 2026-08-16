//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1261/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1261(t1322: f64, t7353: f64, t123: f64, t1316: f64, t1324: f64, t14570: f64, t14571: f64, t14575: f64, t14601: f64, t18453: f64, t18481: f64, t18939: f64, t2180: f64, t21827: f64, t22063: f64, t22074: f64, t2209: f64, t22097: f64, t22113: f64, t2255: f64, t2258: f64, t24: f64, t2730: f64, t2733: f64, t295: f64, t315: f64, t317: f64, t342: f64, t346: f64, t5721: f64, t5934: f64, t6009: f64, t6021: f64, t7102: f64, t769: f64, t787: f64, t7882: f64, t790: f64) -> f64 {
    let t22120 = t7353 * t1322;
    let t22123 = 4.0_f64 * t24 * t18939 * t6009 + 3.0_f64 * t1316 * t790 * t2730 * t342 + 2.0_f64 * t346 * t14601 * t7882 + 9.0_f64 * t1316 * t2258 * t7102 - 5.4655730795145296e-05_f64 * t18453 + 0.020267214298646783_f64 * t123 * t315 * t21827 * t317 + t14570 + 0.17961351015381913_f64 * t14571 + 0.0004919015771563077_f64 * t14575 + 9.0_f64 * t1316 * t790 * t787 * t2209 + 18.0_f64 * t2180 * t790 * t18481 - 2.0_f64 * t346 * t6021 * t5934 + t346 * t2258 * t2730 + 9.0_f64 * t1316 * t790 * t2255 * t769 + (t22063 + t22074 + t22097 + t22113) * t295 + 9.0_f64 * t1316 * t2733 * t5721 - t346 * t22120 * t1324;
    t22123
}
