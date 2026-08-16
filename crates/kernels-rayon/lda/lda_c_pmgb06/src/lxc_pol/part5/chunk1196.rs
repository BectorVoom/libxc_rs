//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1196/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1196(t4359: f64, t7344: f64, t23: f64, t2695: f64, t11674: f64, t11678: f64, t15102: f64, t15106: f64, t15121: f64, t18940: f64, t2255: f64, t2276: f64, t2308: f64, t342: f64, t346: f64, t4355: f64, t4358: f64, t5583: f64, t6007: f64, t6021: f64, t7099: f64, t783: f64, t7881: f64, t8163: f64, t8177: f64, t8180: f64, t8184: f64, t8189: f64, t8208: f64) -> f64 {
    let t21628 = t4359 * t7344;
    let t21633 = t2695 * t23;
    let t21648 = 6.0_f64 * t5583 * t6007 * t7881 * t342 - 9.0_f64 * t18940 * t4355 - 0.03592270203076383_f64 * t15102 - 0.03592270203076383_f64 * t15106 + 18.0_f64 * t4358 * t21628 + 0.012203831437512505_f64 * t11674 - 0.020146007452401596_f64 * t11678 + 18.0_f64 * t21633 * t2276 - 2.0_f64 * t346 * t2308 * t2255 * t783 - 2.0_f64 * t346 * t6021 * t7099 - 0.15965645347006147_f64 * t15121 - 0.01197423401025461_f64 * t8163 - t8177 - 4.569219094474146e-06_f64 * t8180 - t8184 + 0.05987117005127304_f64 * t8189 + 0.19513566535229734_f64 * t8208;
    t21648
}
