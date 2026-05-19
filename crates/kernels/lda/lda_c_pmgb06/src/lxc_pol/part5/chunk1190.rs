//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1190/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1190<F: Float>(t377: F, t7326: F, t2247: F, t5858: F, t7344: F, t11470: F, t11485: F, t18815: F, t21317: F, t21361: F, t21366: F, t21367: F, t21369: F, t2209: F, t2248: F, t2695: F, t342: F, t5874: F, t7277: F, t7306: F, t8428: F, t8439: F) -> (F, F) {
    let t21506 = t7326 * t377;
    let t21558 = t2247 * t5858 * t7344;
    let t21568 = t21317 + t8428 - t21361 + F::new(6.89702) * t11485 - F::new(5.172765) * t18815 + t21366 - t21367 + F::cast_from(1.7881162962962962_f64) * t8439 - t21369 + F::new(5.172765) * t2247 * t2248 * t7306 * t342 - F::new(5.172765) * t21558 + F::new(103.4553) * t2247 * t11470 * t7277 * t342 - F::new(62.07318) * t2247 * t5874 * t2695 * t2209;
    (t21506, t21568)
}
