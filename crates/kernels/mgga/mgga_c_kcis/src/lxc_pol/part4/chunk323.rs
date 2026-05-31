//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 323/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk323<F: Float>(t1195: F, t388: F, t382: F, t1133: F, t358: F, t387: F, t1167: F, t1173: F, t1177: F, t1181: F, t1185: F, t1190: F) -> (F, F, F, F, F, F) {
    let t1196 = t1195 * t388;
    let t1197 = t382 * t1196;
    let t1199 = t358 * t1133;
    let t1200 = t387 * t1199;
    let t1201 = t382 * t1200;
    let t1203 = t1167 / F::cast_from(16.0_f64) - t1173 / F::cast_from(16.0_f64) - t1177 / F::cast_from(6.0_f64) + t1181 / F::cast_from(24.0_f64) - t1185 / F::cast_from(256.0_f64) + t1190 / F::cast_from(256.0_f64) + t1197 / F::cast_from(48.0_f64) - t1201 / F::cast_from(192.0_f64);
    (t1196, t1197, t1199, t1200, t1201, t1203)
}
