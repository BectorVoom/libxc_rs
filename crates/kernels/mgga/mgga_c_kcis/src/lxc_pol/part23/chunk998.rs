//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 998/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk998<F: Float>(t17484: F, t17488: F, t17491: F, t17494: F, t17497: F, t17499: F, t17502: F, t17506: F, t17510: F, t17512: F, t17515: F, t17518: F, t17521: F, t17693: F, t17695: F, t17698: F, t17700: F, t17704: F) -> F {
    let t18350 = F::new(0.101171875e-1) * t17484 - F::new(0.9375e-1) * t17488 + F::new(0.125e0) * t17491 - F::new(0.16666666666666666667e0) * t17494 + F::new(0.25e0) * t17497 + F::new(0.14388888888888888889e0) * t17499 - F::new(0.53958333333333333333e-1) * t17502 - F::new(0.14388888888888888889e0) * t17506 - F::new(0.5625e0) * t17510 - F::new(0.13489583333333333333e-1) * t17512 + F::new(0.20234375e-1) * t17515 - F::new(0.20234375e-1) * t17518 + F::new(0.55555555555555555557e-1) * t17521 + F::new(0.9375e-1) * t17693 + F::new(0.101171875e-1) * t17695 + F::new(0.41666666666666666666e-1) * t17698 - F::new(0.25e0) * t17700 + F::new(0.101171875e-1) * t17704;
    t18350
}
