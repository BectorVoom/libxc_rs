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
    let t22572 = -F::new(0.12) * t11854 * t13631 * t20813 - F::new(0.02666666666666667) * t22526 + F::new(0.09597777777777777) * t13710 - t13715 - F::new(0.006913580246913581) * t25 * t10216 * t21811 + F::new(0.017777777777777778) * t2061 * t3587 * t21815 + F::new(0.013333333333333334) * t25 * t589 * t21820 - F::new(0.0022222222222222222) * t25 * t1371 * t21825 + F::new(0.24) * t2061 * t589 * t21219 + F::new(0.04) * t25 * t589 * t21231 - F::new(0.08) * t2061 * t589 * t21235 - F::new(0.08) * t25 * t1371 * t21794 - F::new(0.08) * t2061 * t1371 * t20911 - F::new(0.006666666666666667) * t25 * t1371 * t21207 + F::new(0.013333333333333334) * t2061 * t1371 * t21211 + F::new(0.16) * t25 * t589 * t21777 + F::new(0.035555555555555556) * t25 * t3587 * t20907 + F::new(0.11197407407407407) * t13731 + F::new(0.09597777777777777) * t13736 + F::new(0.0044444444444444444) * t22570;
    t22572
}
