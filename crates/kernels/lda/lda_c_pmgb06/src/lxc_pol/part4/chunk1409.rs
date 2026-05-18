//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1409/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1409<F: Float>(t1730: F, t2526: F, t12804: F, t16548: F, t16550: F, t16555: F, t16557: F, t16559: F, t16560: F, t16561: F, t16562: F, t16566: F, t16568: F, t16569: F, t16573: F, t16574: F) -> F {
    let t18244 = t2526 * t1730;
    let t18247 = -t16548 + t16550 - t16555 + t16557 - t16559 + t16560 + F::new(0.033245444444444446) * t18244 - t16561 - t16562 + t16566 + t16568 + F::new(16.0) / F::new(81.0) * t12804 - t16569 - t16573 - t16574;
    t18247
}
