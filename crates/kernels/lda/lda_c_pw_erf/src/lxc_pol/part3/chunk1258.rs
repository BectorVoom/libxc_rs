//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1258/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1258<F: Float>(t5446: F, t646: F, t1426: F, t1901: F, t11941: F, t11943: F, t11945: F, t11947: F, t11949: F, t11953: F, t11955: F, t11956: F, t11957: F, t11958: F, t11960: F) -> F {
    let t14978 = t5446 * t646;
    let t14979 = F::new(0.09973633333333333) * t14978;
    let t14980 = t1901 * t1426;
    let t14982 = t11941 - t11943 + t14979 + F::new(0.09973633333333333) * t14980 - t11945 - t11947 - t11949 + t11953 - t11955 + t11956 - t11957 - t11958 + t11960;
    t14982
}
