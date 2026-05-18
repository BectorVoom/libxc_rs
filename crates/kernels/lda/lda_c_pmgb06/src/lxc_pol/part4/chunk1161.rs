//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1161/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1161<F: Float>(t11904: F, t6630: F, t12497: F, t5068: F, t6629: F, t2088: F, t764: F, t337: F, t5069: F, t2489: F, t3198: F, t1444: F, t6292: F) -> (F, F, F, F, F, F, F) {
    let t15270 = F::new(8.0) / F::new(45.0) * t11904 * t6630;
    let t15273 = F::new(8.0) / F::new(45.0) * t5068 * t12497 * t6629;
    let t15274 = t764 * t2088;
    let t15275 = t15274 * t337;
    let t15278 = F::new(8.0) / F::new(45.0) * t5068 * t5069 * t15275;
    let t15280 = F::new(2.0) / F::new(45.0) * t3198 * t2489;
    let t15282 = F::new(4.0) / F::new(45.0) * t1444 * t6292;
    (t15270, t15273, t15274, t15275, t15278, t15280, t15282)
}
