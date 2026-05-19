//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 301/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk301<F: Float>(t973: F, t991: F, t31: F, t4: F, t474: F, t155: F, t318: F, t174: F, t335: F, t379: F, t378: F, t80: F) -> (F, F, F, F, F, F) {
    let t992 = t973 * t991;
    let t996 = t4 * t474 * t31;
    let t997 = F::cast_from(0.0014764770444444443_f64) * t996;
    let t998 = t155 * t318;
    let t1000 = t174 * t998 * t335;
    let t1001 = F::cast_from(0.035616666666666665_f64) * t1000;
    let t1005 = t155 * t379;
    let t1009 = t378 * t80;
    (t992, t997, t998, t1001, t1005, t1009)
}
