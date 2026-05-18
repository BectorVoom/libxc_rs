//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1085/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1085<F: Float>(t1423: F, t7690: F, t2477: F, t5220: F, t1447: F, t7685: F, t10247: F, t12622: F, t12633: F, t1420: F, t1972: F, t19870: F, t2501: F, t2948: F, t439: F, t442: F, t444: F, t5187: F, t6114: F, t6523: F, t7524: F, t7525: F, t7584: F, t7585: F) -> F {
    let t20062 = t1423 * t7690;
    let t20064 = t5220 * t2477;
    let t20066 = t1447 * t7685;
    let t20068 = t12622 - F::new(2.0) / F::new(15.0) * t5187 * t2501 + F::new(2.0) / F::new(15.0) * t439 * t12633 * t6523 - t1420 * t7525 / F::new(15.0) - t439 * t2948 * t7524 / F::new(15.0) + t439 * t442 * t444 * t19870 / F::new(45.0) + F::new(8.0) / F::new(81.0) * t1420 * t7585 + F::new(8.0) / F::new(81.0) * t439 * t10247 * t7584 + t1972 * t6114 / F::new(5.0) + F::new(2.0) / F::new(45.0) * t20062 + F::new(4.0) / F::new(45.0) * t20064 + F::new(2.0) / F::new(15.0) * t20066;
    t20068
}
