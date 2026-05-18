//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 454/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk454<F: Float>(t2146: F, t577: F, t1472: F, t826: F, t473: F, t573: F) -> (F, F, F) {
    let t2148 = F::new(4.0) / F::new(45.0) * t2146 * t577;
    let t2150 = F::new(4.0) / F::new(45.0) * t1472 * t826;
    let t2151 = t473 * t573;
    (t2148, t2150, t2151)
}
