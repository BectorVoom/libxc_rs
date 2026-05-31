//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 290/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk290<F: Float>(t1200: F, t382: F, t1167: F, t1173: F, t1177: F, t1181: F, t1185: F, t1190: F, t1197: F) -> (F, F) {
    let t1201 = t382 * t1200;
    let t1203 = t1167 / F::cast_from(16.0_f64) - t1173 / F::cast_from(16.0_f64) - t1177 / F::cast_from(6.0_f64) + t1181 / F::cast_from(24.0_f64) - t1185 / F::cast_from(256.0_f64) + t1190 / F::cast_from(256.0_f64) + t1197 / F::cast_from(48.0_f64) - t1201 / F::cast_from(192.0_f64);
    (t1201, t1203)
}
