//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 809/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk809<F: Float>(t4160: F, t4163: F, t4165: F, t4172: F, t4175: F, t4254: F, t4258: F, t5440: F, t7222: F, t7294: F, t7298: F, t7300: F) -> F {
    let t7400 = -F::new(0.09451622166942335) * t7298 - F::new(0.005926167098672845) * t7222 - F::new(0.07184540406152766) * t5440 + F::new(0.02694202652307287) * t7294 + F::new(0.09451622166942335) * t7300 - t4160 - t4163 - t4165 + t4172 + t4175 + t4254 + t4258;
    t7400
}
