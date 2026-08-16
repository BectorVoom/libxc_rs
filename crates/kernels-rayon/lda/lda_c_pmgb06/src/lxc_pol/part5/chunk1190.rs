//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1190/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1190(t377: f64, t7326: f64, t2247: f64, t5858: f64, t7344: f64, t11470: f64, t11485: f64, t18815: f64, t21317: f64, t21361: f64, t21366: f64, t21367: f64, t21369: f64, t2209: f64, t2248: f64, t2695: f64, t342: f64, t5874: f64, t7277: f64, t7306: f64, t8428: f64, t8439: f64) -> (f64, f64) {
    let t21506 = t7326 * t377;
    let t21558 = t2247 * t5858 * t7344;
    let t21568 = t21317 + t8428 - t21361 + 6.89702_f64 * t11485 - 5.172765_f64 * t18815 + t21366 - t21367 + 1.7881162962962962_f64 * t8439 - t21369 + 5.172765_f64 * t2247 * t2248 * t7306 * t342 - 5.172765_f64 * t21558 + 103.4553_f64 * t2247 * t11470 * t7277 * t342 - 62.07318_f64 * t2247 * t5874 * t2695 * t2209;
    (t21506, t21568)
}
