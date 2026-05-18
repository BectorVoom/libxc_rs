//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1231/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1231<F: Float>(t12804: F, t20174: F, t20177: F, t20182: F, t20186: F, t20189: F, t20191: F, t20194: F, t20197: F, t20199: F, t20201: F, t20204: F) -> F {
    let t21972 = -t20174 + t20177 + t20182 - t20186 - t20189 - t20191 - t20194 - t20197 - t20199 - t20201 + F::new(8.0) / F::new(27.0) * t12804 + t20204;
    t21972
}
