//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1236/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1236<F: Float>(t13115: F, t6446: F, t6748: F, t13035: F, t7749: F, t20711: F, t593: F, t13122: F, t4506: F, t13966: F, t20712: F, t13812: F) -> (F, F, F, F, F, F) {
    let t22237 = F::new(64.0) / F::new(15.0) * t13115 * t6748 * t6446;
    let t22239 = F::new(16.0) / F::new(15.0) * t13035 * t7749;
    let t22240 = t20711 * t593;
    let t22243 = F::new(16.0) / F::new(15.0) * t4506 * t13122 * t22240;
    let t22246 = F::new(8.0) / F::new(5.0) * t4506 * t13966 * t20712;
    let t22249 = F::new(8.0) / F::new(3.0) * t4506 * t13812 * t20712;
    (t22237, t22239, t22240, t22243, t22246, t22249)
}
