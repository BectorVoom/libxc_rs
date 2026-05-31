//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 251/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk251<F: Float>(t1305: F, t1309: F, t1320: F, t1319: F, t1410: F, t456: F) -> (F, F, F) {
    let t1414 = F::cast_from(0.41275e-2_f64) * t1305;
    let t1416 = F::cast_from(0.1982e-1_f64) * t1320 - t1414 - F::cast_from(0.41275e-2_f64) * t1309;
    let t1419 = t1410 * t1319 / F::cast_from(4.0_f64) + t456 * t1416 / F::cast_from(2.0_f64);
    (t1414, t1416, t1419)
}
