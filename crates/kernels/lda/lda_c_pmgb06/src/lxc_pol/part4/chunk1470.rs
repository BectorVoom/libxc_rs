//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1470/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1470<F: Float>(t26: F, t2732: F, t329: F, t2407: F, t247: F, t1156: F, t123: F, t2422: F, t10903: F, t10905: F, t1167: F, t14663: F, t14666: F, t14669: F, t14672: F, t18437: F, t305: F, t6939: F, t726: F) -> (F, F) {
    let t18939 = t26 * t2732;
    let t18940 = t329 * t18939;
    let t18954 = t247 * t2407;
    let t18969 = t123 * t1156 * t2422;
    let t18973 = -F::new(0.2133002709687175) * t14663 + F::new(0.31995040645307626) * t18954 - F::new(0.031835665774679375) * t123 * t305 * t18437 - F::new(0.031835665774679375) * t123 * t1167 * t2422 + F::new(1.0376068845080684) * t14666 + F::new(1.0376068845080684) * t14669 + F::new(0.10611888591559791) * t14672 - F::new(0.06367133154935875) * t123 * t726 * t6939 + F::new(0.10611888591559791) * t18969 + F::new(0.31995040645307626) * t10903 - F::new(2.55960325162461) * t10905;
    (t18940, t18973)
}
