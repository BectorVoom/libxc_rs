//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1262/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1262<F: Float>(t707: F, t7934: F, t123: F, t317: F, t740: F, t7425: F, t10599: F, t10603: F, t10609: F, t10614: F, t10617: F, t10635: F, t10640: F, t10643: F, t10646: F, t15086: F, t15089: F, t15136: F, t18095: F, t2180: F, t2258: F, t5583: F, t6012: F, t6018: F, t6031: F) -> F {
    let t22128 = t707 * t7934;
    let t22135 = t123 * t740 * t7425 * t317;
    let t22146 = t10599 - F::new(6.0) * t5583 * t15136 * t6012 + F::cast_from(0.019957056683757683_f64) * t22128 - t10603 + F::cast_from(0.004067943812504169_f64) * t10609 - t10614 + t10617 - F::cast_from(0.0008717022455366076_f64) * t10635 + t10640 - t10643 - F::cast_from(0.006715335817467199_f64) * t10646 - F::cast_from(0.054045904796391424_f64) * t22135 + F::new(18.0) * t2180 * t2258 * t6031 + F::new(36.0) * t6018 * t15089 - F::new(9.0) * t5583 * t18095 - F::new(18.0) * t6018 * t15086;
    t22146
}
