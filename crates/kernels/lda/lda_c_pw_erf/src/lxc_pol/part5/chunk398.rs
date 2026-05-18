//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 398/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk398<F: Float>(t34: F, t408: F, t1820: F, t1823: F, t1826: F, t348: F, t352: F, t462: F) -> F {
    let t1829 = t408 * t34;
    let t1832 = -t1820 * t348 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t1823 * t462 - t1826 * t352 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t1829 * t462;
    t1832
}
