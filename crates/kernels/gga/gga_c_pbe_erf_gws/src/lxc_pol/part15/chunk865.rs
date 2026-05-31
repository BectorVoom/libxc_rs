//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 865/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk865<F: Float>(t7373: F, t7411: F, t598: F, t186: F, t185: F, t5278: F, t5281: F, t5285: F, t5315: F, t1006: F, t1673: F, t5317: F) -> (F, F, F, F, F, F, F) {
    let t7412 = t7373 + t7411;
    let t7413 = t598 * t7412;
    let t7414 = t186 * t7413;
    let t7416 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t7414;
    let t7417 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5278;
    let t7418 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5281;
    let t7419 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t5285;
    let t7420 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t5315;
    let t7421 = t1006 * t1673;
    let t7422 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t7421;
    let t7423 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5317;
    (t7416, t7417, t7418, t7419, t7420, t7422, t7423)
}
