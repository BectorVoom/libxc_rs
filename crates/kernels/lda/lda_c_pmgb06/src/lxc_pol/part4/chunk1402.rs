//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1402/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1402<F: Float>(t1680: F, t2527: F, t12304: F, t12307: F, t12308: F, t12310: F, t12312: F, t16297: F, t16299: F, t16300: F, t16301: F, t16302: F, t16303: F, t16306: F, t16308: F, t16310: F) -> F {
    let t18225 = t2527 * t1680;
    let t18227 = -t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 + F::new(4.0) / F::new(3.0) * t12304 + t12307 + F::new(4.0) / F::new(9.0) * t12308 + F::new(16.0) / F::new(9.0) * t12310 + F::new(4e-21) * t12312 - t16308 + t16310 - F::new(2.0) / F::new(27.0) * t18225;
    t18227
}
