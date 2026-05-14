//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 951/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk951<F: Float>(t12840: F, t1602: F, t161: F, t166: F, t4841: F, t486: F, t4847: F, t493: F, t5447: F, t4857: F, t5463: F, t2965: F, t439: F, t5482: F, t1444: F, t5451: F) -> (F, F, F, F, F, F) {
    let t12844 = t161 * t166 * t12840 * t1602 / 5.0;
    let t12846 = 2.0 / 15.0 * t486 * t4841;
    let t12849 = 2.0 / 15.0 * t493 * t5447 * t4847;
    let t12852 = 2.0 / 3.0 * t493 * t5463 * t4857;
    let t12855 = 2.0 / 15.0 * t439 * t5482 * t2965;
    let t12857 = 2.0 / 15.0 * t1444 * t5451;
    (t12844, t12846, t12849, t12852, t12855, t12857)
}
