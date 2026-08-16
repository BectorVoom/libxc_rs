//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1262/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1262(t707: f64, t7934: f64, t123: f64, t317: f64, t740: f64, t7425: f64, t10599: f64, t10603: f64, t10609: f64, t10614: f64, t10617: f64, t10635: f64, t10640: f64, t10643: f64, t10646: f64, t15086: f64, t15089: f64, t15136: f64, t18095: f64, t2180: f64, t2258: f64, t5583: f64, t6012: f64, t6018: f64, t6031: f64) -> f64 {
    let t22128 = t707 * t7934;
    let t22135 = t123 * t740 * t7425 * t317;
    let t22146 = t10599 - 6.0_f64 * t5583 * t15136 * t6012 + 0.019957056683757683_f64 * t22128 - t10603 + 0.004067943812504169_f64 * t10609 - t10614 + t10617 - 0.0008717022455366076_f64 * t10635 + t10640 - t10643 - 0.006715335817467199_f64 * t10646 - 0.054045904796391424_f64 * t22135 + 18.0_f64 * t2180 * t2258 * t6031 + 36.0_f64 * t6018 * t15089 - 9.0_f64 * t5583 * t18095 - 18.0_f64 * t6018 * t15086;
    t22146
}
