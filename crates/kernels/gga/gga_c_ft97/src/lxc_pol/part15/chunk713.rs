//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 713/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk713<F: Float>(t20223: F, t20273: F, t20304: F, t20467: F, t103: F, t20461: F, t108: F, t19970: F, t19972: F, t20163: F, t20269: F, t20396: F, t20400: F, t20404: F, t20408: F, t20420: F, t4415: F, t4501: F, t4621: F, t88: F, t948: F, t984: F) -> (F, F, F) {
    let t20469 = t20223 + t20273 + t20304 + t20467;
    let t20471 = t20461 * t103;
    let t20479 = -t108 * t19970 - F::cast_from(2.0_f64) * t108 * t19972 - t108 * t20163 - t20469 * t88 - F::cast_from(3.0_f64) * t4415 * t984 - F::cast_from(3.0_f64) * t4501 * t984 - F::cast_from(3.0_f64) * t4621 * t948 + F::cast_from(12.0_f64) * t20269 - F::cast_from(2.0_f64) * t20396 - F::cast_from(6.0_f64) * t20400 - F::cast_from(12.0_f64) * t20404 + F::cast_from(12.0_f64) * t20408 - F::cast_from(6.0_f64) * t20420 + F::cast_from(2.0_f64) * t20471;
    (t20469, t20471, t20479)
}
