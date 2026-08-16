//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 896/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk896<F: Float>(t7311: F, t7315: F, t7319: F, t7323: F, t7330: F, t7333: F, t7336: F, t7339: F, t7383: F, t7387: F, t7390: F, t7394: F) -> F {
    let t7565 = -F::cast_from(0.20833333333333333333e-1_f64) * t7311 + F::cast_from(0.625e-1_f64) * t7315 - F::cast_from(0.20234375e-1_f64) * t7319 - F::cast_from(0.101171875e-1_f64) * t7323 - F::cast_from(0.34173611111111111111e0_f64) * t7330 + F::cast_from(0.14388888888888888889e0_f64) * t7333 + F::cast_from(0.5e0_f64) * t7336 - F::cast_from(0.125e0_f64) * t7339 + F::cast_from(0.9375e-1_f64) * t7383 + F::cast_from(0.91666666666666666667e0_f64) * t7387 - F::cast_from(0.33333333333333333334e0_f64) * t7390 + F::cast_from(0.1875e0_f64) * t7394;
    t7565
}
