//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1228/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1228(t2249: f64, t384: f64, t387: f64, t10551: f64, t10582: f64, t10648: f64, t11717: f64, t11740: f64, t1312: f64, t1316: f64, t14247: f64, t14257: f64, t14270: f64, t14497: f64, t14511: f64, t14521: f64, t14551: f64, t2180: f64, t2311: f64, t295: f64, t346: f64, t3659: f64, t4053: f64, t4358: f64, t4398: f64, t5583: f64, t5721: f64, t61: f64, t790: f64, t8189: f64, t8202: f64, t8206: f64, t8208: f64, t8211: f64) -> f64 {
    let t14561 = t387 * t384 * t2249;
    let t14564 = 0.17961351015381913_f64 * t8189 - 0.01197423401025461_f64 * t8202 - 0.03592270203076383_f64 * t8206 + 0.585406996056892_f64 * t8208 + t8211 + (t11717 + t11740 + t14247 + t14257) * t295 + 3.0_f64 * t1316 * t3659 * t2311 - 3.0_f64 * t346 * t4398 * t4053 + 9.0_f64 * t1316 * t1312 * t5721 - 6.0_f64 * t346 * t14270 * t10551 + (t14497 + t14511 + t14521 + t14551) * t61 + 18.0_f64 * t2180 * t790 * t10648 - 9.0_f64 * t5583 * t10582 + 18.0_f64 * t4358 * t14561;
    t14564
}
