//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1159/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1159<F: Float>(t17801: F, t17809: F, t2002: F, t6775: F, t2979: F, t493: F, t7538: F, t1380: F, t16856: F, t764: F, t1444: F, t7539: F) -> (F, F, F, F, F, F) {
    let t20919 = F::new(4.0) / F::new(45.0) * t17801;
    let t20920 = F::new(4.0) / F::new(27.0) * t17809;
    let t20922 = t2002 * t6775 / F::new(15.0);
    let t20925 = t493 * t2979 * t7538 / F::new(15.0);
    let t20929 = t493 * t1380 * t16856 * t764 / F::new(15.0);
    let t20931 = t1444 * t7539 / F::new(15.0);
    (t20919, t20920, t20922, t20925, t20929, t20931)
}
