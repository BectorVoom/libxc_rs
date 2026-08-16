//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1256/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1256(t331: f64, t7427: f64, t7419: f64, t10216: f64, t11854: f64, t13631: f64, t1371: f64, t13710: f64, t13715: f64, t13731: f64, t13736: f64, t2061: f64, t20813: f64, t20907: f64, t20911: f64, t21207: f64, t21211: f64, t21219: f64, t21231: f64, t21235: f64, t21777: f64, t21794: f64, t21811: f64, t21815: f64, t21820: f64, t21825: f64, t25: f64, t3587: f64, t589: f64) -> f64 {
    let t22526 = t331 * t7427;
    let t22570 = t331 * t7419;
    let t22572 = -0.12_f64 * t11854 * t13631 * t20813 - 0.02666666666666667_f64 * t22526 + 0.09597777777777777_f64 * t13710 - t13715 - 0.006913580246913581_f64 * t25 * t10216 * t21811 + 0.017777777777777778_f64 * t2061 * t3587 * t21815 + 0.013333333333333334_f64 * t25 * t589 * t21820 - 0.0022222222222222222_f64 * t25 * t1371 * t21825 + 0.24_f64 * t2061 * t589 * t21219 + 0.04_f64 * t25 * t589 * t21231 - 0.08_f64 * t2061 * t589 * t21235 - 0.08_f64 * t25 * t1371 * t21794 - 0.08_f64 * t2061 * t1371 * t20911 - 0.006666666666666667_f64 * t25 * t1371 * t21207 + 0.013333333333333334_f64 * t2061 * t1371 * t21211 + 0.16_f64 * t25 * t589 * t21777 + 0.035555555555555556_f64 * t25 * t3587 * t20907 + 0.11197407407407407_f64 * t13731 + 0.09597777777777777_f64 * t13736 + 0.0044444444444444444_f64 * t22570;
    t22572
}
