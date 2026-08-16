//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1409/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1409<F: Float>(t17484: F, t17488: F, t17491: F, t17494: F, t17497: F, t17499: F, t17502: F, t17506: F, t17510: F, t17512: F, t17515: F, t17518: F, t17521: F, t17693: F, t17695: F, t17698: F, t17700: F, t17704: F) -> F {
    let t18350 = F::cast_from(0.101171875e-1_f64) * t17484 - F::cast_from(0.9375e-1_f64) * t17488 + F::cast_from(0.125e0_f64) * t17491 - F::cast_from(0.16666666666666666667e0_f64) * t17494 + F::cast_from(0.25e0_f64) * t17497 + F::cast_from(0.14388888888888888889e0_f64) * t17499 - F::cast_from(0.53958333333333333333e-1_f64) * t17502 - F::cast_from(0.14388888888888888889e0_f64) * t17506 - F::cast_from(0.5625e0_f64) * t17510 - F::cast_from(0.13489583333333333333e-1_f64) * t17512 + F::cast_from(0.20234375e-1_f64) * t17515 - F::cast_from(0.20234375e-1_f64) * t17518 + F::cast_from(0.55555555555555555557e-1_f64) * t17521 + F::cast_from(0.9375e-1_f64) * t17693 + F::cast_from(0.101171875e-1_f64) * t17695 + F::cast_from(0.41666666666666666666e-1_f64) * t17698 - F::cast_from(0.25e0_f64) * t17700 + F::cast_from(0.101171875e-1_f64) * t17704;
    t18350
}
