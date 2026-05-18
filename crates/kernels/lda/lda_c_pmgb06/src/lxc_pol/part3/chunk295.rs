//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 295/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk295<F: Float>(t667: F, t993: F, t942: F, t945: F, t947: F, t951: F, t953: F, t955: F) -> (F, F) {
    let t994 = t993 * t667;
    let t1003 = -F::new(0.7843833333333333) * t942 + F::new(1.5687666666666666) * t945 + F::new(0.6886333333333333) * t947 + F::new(0.14025833333333335) * t951 + F::new(0.2805166666666667) * t953 + F::new(0.17365833333333333) * t955;
    (t994, t1003)
}
