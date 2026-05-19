//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1383/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1383<F: Float>(t16126: F, t16226: F, t16244: F, t16251: F, t16253: F, t16256: F, t16259: F, t16262: F, t16266: F, t16269: F, t16273: F, t16276: F, t17915: F, t601: F) -> F {
    let t17919 = -t16126 - t16226 + t16251 - t16253 + t16256 + t16259 + t16262 - t16266 - t16269 - t16273 - t16276 - F::new(0.3109e-1) * t17915 * t601 - F::cast_from(0.19751789702565206229e-1_f64) * t16244;
    t17919
}
