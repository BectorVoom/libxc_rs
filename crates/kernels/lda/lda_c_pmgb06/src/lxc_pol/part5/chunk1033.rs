//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1033/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1033<F: Float>(t350: F, t7602: F, t7609: F, t337: F, t7295: F, t9525: F, t2909: F, t36: F, t17054: F, t17059: F, t17061: F, t17066: F, t19316: F, t19319: F, t19322: F, t19324: F, t19326: F, t19334: F, t19338: F, t19342: F, t19346: F, t19351: F, t19356: F, t19360: F, t19364: F) -> (F, F, F, F, F) {
    let t19366 = t350 * t7602;
    let t19368 = t350 * t7609;
    let t19371 = t9525 * t7295 * t337;
    let t19373 = t36 * t2909 * t19371;
    let t19375 = F::new(0.034005) * t19316 - F::new(0.02267) * t19319 + F::new(0.006297222222222222) * t19322 - F::new(0.003778333333333333) * t19324 - F::new(0.0006297222222222223) * t19326 - F::new(0.0018891666666666666) * t17054 - F::new(0.005037777777777778) * t17059 + F::new(0.0016792592592592592) * t17061 + F::new(0.002518888888888889) * t17066 - F::new(0.04534) * t19334 - F::new(0.06801) * t19338 - F::new(0.011335) * t19342 + F::new(0.02267) * t19346 - F::new(0.003778333333333333) * t19351 + F::new(0.0012594444444444445) * t19356 + F::new(0.003778333333333333) * t19360 - F::new(0.007556666666666666) * t19364 - F::new(0.0012594444444444445) * t19366 + F::new(0.003778333333333333) * t19368 - F::new(0.02518888888888889) * t19373;
    (t19366, t19368, t19371, t19373, t19375)
}
