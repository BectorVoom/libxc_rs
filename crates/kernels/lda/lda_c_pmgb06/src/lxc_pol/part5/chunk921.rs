//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 921/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk921<F: Float>(t107: F, t1180: F, t2164: F, t2786: F, t902: F, t4844: F, t486: F, t3005: F, t831: F, t1730: F, t2025: F, t2021: F) -> (F, F, F, F, F, F) {
    let t11744 = t107 * t1180 * t2164;
    let t11745 = F::new(3.9861630686838536) * t11744;
    let t11747 = t107 * t2786 * t902;
    let t11757 = t486 * t4844;
    let t11758 = t11757 / F::new(45.0);
    let t11777 = t831 * t3005;
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    (t11745, t11747, t11758, t11777, t11796, t11798)
}
