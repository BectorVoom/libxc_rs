//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1256/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1256<F: Float>(t331: F, t7427: F, t7419: F, t10216: F, t11854: F, t13631: F, t1371: F, t13710: F, t13715: F, t13731: F, t13736: F, t2061: F, t20813: F, t20907: F, t20911: F, t21207: F, t21211: F, t21219: F, t21231: F, t21235: F, t21777: F, t21794: F, t21811: F, t21815: F, t21820: F, t21825: F, t25: F, t3587: F, t589: F) -> F {
    let t22526 = t331 * t7427;
    let t22570 = t331 * t7419;
    let t22572 = -F::cast_from(0.12_f64) * t11854 * t13631 * t20813 - F::cast_from(0.02666666666666667_f64) * t22526 + F::cast_from(0.09597777777777777_f64) * t13710 - t13715 - F::cast_from(0.006913580246913581_f64) * t25 * t10216 * t21811 + F::cast_from(0.017777777777777778_f64) * t2061 * t3587 * t21815 + F::cast_from(0.013333333333333334_f64) * t25 * t589 * t21820 - F::cast_from(0.0022222222222222222_f64) * t25 * t1371 * t21825 + F::cast_from(0.24_f64) * t2061 * t589 * t21219 + F::cast_from(0.04_f64) * t25 * t589 * t21231 - F::cast_from(0.08_f64) * t2061 * t589 * t21235 - F::cast_from(0.08_f64) * t25 * t1371 * t21794 - F::cast_from(0.08_f64) * t2061 * t1371 * t20911 - F::cast_from(0.006666666666666667_f64) * t25 * t1371 * t21207 + F::cast_from(0.013333333333333334_f64) * t2061 * t1371 * t21211 + F::cast_from(0.16_f64) * t25 * t589 * t21777 + F::cast_from(0.035555555555555556_f64) * t25 * t3587 * t20907 + F::cast_from(0.11197407407407407_f64) * t13731 + F::cast_from(0.09597777777777777_f64) * t13736 + F::cast_from(0.0044444444444444444_f64) * t22570;
    t22572
}
