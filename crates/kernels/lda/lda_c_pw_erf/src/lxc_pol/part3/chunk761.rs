//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 761/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk761<F: Float>(t1390: F, t1392: F, t784: F, t1440: F, t1325: F, t188: F, t473: F, t34: F, t529: F, t542: F, t2067: F, t565: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4952 = t1390 * t784 * t1392;
    let t4953 = t1440 * t4952;
    let t4955 = F::new(8.0) / F::new(15.0) * t1325 * t4953;
    let t4956 = t473 * t188;
    let t4957 = t529 * t34;
    let t4958 = t4957 * t542;
    let t4959 = t4956 * t4958;
    let t4961 = F::new(8.0) / F::new(15.0) * t1325 * t4959;
    let t4963 = F::new(4.0) / F::new(15.0) * t565 * t2067;
    (t4952, t4953, t4955, t4956, t4957, t4958, t4959, t4961, t4963)
}
