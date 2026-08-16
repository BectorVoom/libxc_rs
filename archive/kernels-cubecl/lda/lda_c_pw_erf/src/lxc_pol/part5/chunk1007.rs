//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1007/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1007<F: Float>(t571: F, t6374: F, t9278: F, t14240: F, t6384: F, t13455: F, t6388: F, t3802: F, t519: F, t6488: F, t2146: F, t5302: F) -> (F, F, F, F, F) {
    let t16069 = t571 * t9278 * t6374;
    let t16072 = t571 * t14240 * t6384;
    let t16075 = t571 * t13455 * t6388;
    let t16084 = t519 * t3802 * t6488;
    let t16092 = t2146 * t5302;
    (t16069, t16072, t16075, t16084, t16092)
}
