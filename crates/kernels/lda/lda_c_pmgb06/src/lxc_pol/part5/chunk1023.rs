//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1023/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1023<F: Float>(t14016: F, t14465: F, t14467: F, t14472: F, t21050: F, t21052: F, t21055: F, t21059: F, t21061: F, t21065: F, t21066: F, t17886: F, t17890: F, t1444: F, t7715: F, t2979: F, t493: F, t7714: F) -> (F, F, F, F, F) {
    let t21067 = t21050 - t21052 - t21055 - t21059 + 12.0 * t14465 + 4.0 / 3.0 * t21061 + 0.0033101111111111113 * t14467 + t14472 + t21065 - t21066 + t14016;
    let t21068 = 8.0 / 45.0 * t17886;
    let t21069 = 4.0 / 27.0 * t17890;
    let t21071 = 2.0 / 15.0 * t1444 * t7715;
    let t21074 = 2.0 / 15.0 * t493 * t2979 * t7714;
    (t21067, t21068, t21069, t21071, t21074)
}
