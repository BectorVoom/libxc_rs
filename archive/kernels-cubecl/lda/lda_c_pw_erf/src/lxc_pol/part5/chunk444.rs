//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 444/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk444<F: Float>(t2114: F, t786: F, t1298: F, t172: F, t793: F, t184: F) -> (F, F, F, F) {
    let t2116 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2114 * t786;
    let t2118 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1298 * t786;
    let t2119 = t172 * t793;
    let t2120 = t2119 * t184;
    (t2116, t2118, t2119, t2120)
}
