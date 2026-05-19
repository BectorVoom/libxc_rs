//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 932/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk932<F: Float>(t3933: F, t656: F, t1: F, t3921: F, t4166: F, t119: F, t1426: F, t3920: F, t19: F, t2877: F, t646: F, t732: F) -> (F, F, F, F) {
    let t11063 = F::new(8.0) / F::new(9.0) * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    let t11069 = F::cast_from(0.006061752703703704_f64) * t3920 * t119 * t1426;
    let t11073 = F::cast_from(0.0002763148940771605_f64) * t2877 * t19 * t732 * t646;
    (t11063, t11065, t11069, t11073)
}
