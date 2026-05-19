//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 570/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk570<F: Float>(t3440: F, t3445: F, t3449: F, t3453: F, t3455: F, t3457: F, t3461: F, t3467: F, t3469: F, t3471: F, t3475: F, t3478: F) -> F {
    let t3698 = F::cast_from(0.26979166666666666666e-1_f64) * t3440 - F::new(0.20234375e-1) * t3445 - F::cast_from(0.13489583333333333333e-1_f64) * t3449 + F::cast_from(0.101171875e-1_f64) * t3453 + F::cast_from(0.10791666666666666667e0_f64) * t3455 - F::cast_from(0.26979166666666666666e-1_f64) * t3457 - F::new(0.9375e-1) * t3461 + F::new(0.1875e0) * t3467 - F::new(0.5e0) * t3469 + F::new(0.125e0) * t3471 - F::new(0.1875e0) * t3475 + F::new(0.20234375e-1) * t3478;
    t3698
}
