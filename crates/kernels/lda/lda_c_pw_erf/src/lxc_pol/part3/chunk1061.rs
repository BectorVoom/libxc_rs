//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1061/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1061<F: Float>(t3828: F, t4506: F, t4508: F, t1401: F, t1484: F, t3833: F, t833: F, t3837: F, t3974: F, t5151: F, t12373: F, t4488: F, t4494: F) -> (F, F, F, F, F) {
    let t12427 = F::new(8.0) / F::new(15.0) * t4506 * t4508 * t3828;
    let t12428 = t1484 * t1401;
    let t12432 = F::new(8.0) / F::new(9.0) * t4506 * t12428 * t833 * t3833;
    let t12435 = F::new(16.0) / F::new(15.0) * t3974 * t5151 * t3837;
    let t12438 = F::new(8.0) / F::new(15.0) * t4488 * t4494 * t12373;
    (t12427, t12428, t12432, t12435, t12438)
}
