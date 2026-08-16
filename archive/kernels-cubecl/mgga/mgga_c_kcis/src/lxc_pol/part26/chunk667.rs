//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 667/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk667<F: Float>(t7276: F, t7278: F, t7280: F, t7284: F, t7288: F, t7290: F, t7292: F, t7294: F, t7297: F, t7300: F, t7302: F, t7306: F) -> F {
    let t7552 = -F::cast_from(0.44965277777777777777e-2_f64) * t7276 - F::cast_from(0.5e0_f64) * t7278 + F::cast_from(0.125e0_f64) * t7280 - F::cast_from(0.9375e-1_f64) * t7284 - F::cast_from(0.13489583333333333333e-1_f64) * t7288 + F::cast_from(0.10791666666666666667e0_f64) * t7290 - F::cast_from(0.26979166666666666666e-1_f64) * t7292 + F::cast_from(0.20234375e-1_f64) * t7294 - F::cast_from(0.10791666666666666667e0_f64) * t7297 + F::cast_from(0.26979166666666666666e-1_f64) * t7300 - F::cast_from(0.1875e0_f64) * t7302 + F::cast_from(0.101171875e-1_f64) * t7306;
    t7552
}
