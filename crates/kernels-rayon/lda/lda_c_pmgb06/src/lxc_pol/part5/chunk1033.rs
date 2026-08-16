//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1033/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1033(t350: f64, t7602: f64, t7609: f64, t337: f64, t7295: f64, t9525: f64, t2909: f64, t36: f64, t17054: f64, t17059: f64, t17061: f64, t17066: f64, t19316: f64, t19319: f64, t19322: f64, t19324: f64, t19326: f64, t19334: f64, t19338: f64, t19342: f64, t19346: f64, t19351: f64, t19356: f64, t19360: f64, t19364: f64) -> (f64, f64, f64, f64, f64) {
    let t19366 = t350 * t7602;
    let t19368 = t350 * t7609;
    let t19371 = t9525 * t7295 * t337;
    let t19373 = t36 * t2909 * t19371;
    let t19375 = 0.034005_f64 * t19316 - 0.02267_f64 * t19319 + 0.006297222222222222_f64 * t19322 - 0.003778333333333333_f64 * t19324 - 0.0006297222222222223_f64 * t19326 - 0.0018891666666666666_f64 * t17054 - 0.005037777777777778_f64 * t17059 + 0.0016792592592592592_f64 * t17061 + 0.002518888888888889_f64 * t17066 - 0.04534_f64 * t19334 - 0.06801_f64 * t19338 - 0.011335_f64 * t19342 + 0.02267_f64 * t19346 - 0.003778333333333333_f64 * t19351 + 0.0012594444444444445_f64 * t19356 + 0.003778333333333333_f64 * t19360 - 0.007556666666666666_f64 * t19364 - 0.0012594444444444445_f64 * t19366 + 0.003778333333333333_f64 * t19368 - 0.02518888888888889_f64 * t19373;
    (t19366, t19368, t19371, t19373, t19375)
}
