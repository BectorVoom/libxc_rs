//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1134/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1134<F: Float>(t12617: F, t2612: F, t12620: F, t12623: F, t12627: F, t7527: F, t12818: F, t48067: F, t48069: F, t48071: F, t48076: F, t48078: F, t48080: F) -> (F, F, F, F, F, F) {
    let t48082 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t2612 * t12617;
    let t48084 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t2612 * t12620;
    let t48086 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2612 * t12623;
    let t48088 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7527 * t12627;
    let t48090 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2612 * t12818;
    let t48091 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 - t48082 - t48084 + t48086 + t48088 - t48090;
    (t48082, t48084, t48086, t48088, t48090, t48091)
}
