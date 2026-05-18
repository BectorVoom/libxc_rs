//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 930/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk930<F: Float>(t3391: F, t582: F, t211: F, t202: F, t3477: F, t184: F, t619: F, t5208: F, t3345: F, t572: F, t418: F, t1821: F) -> (F, F, F, F) {
    let t10415 = t582 * t3391;
    let t10416 = t211 * t10415;
    let t10417 = F::new(8.0) / F::new(45.0) * t10416;
    let t10418 = t202 * t3477;
    let t10419 = t10418 * t184;
    let t10421 = F::new(4.0) / F::new(15.0) * t10419 * t619;
    let t10423 = F::new(4.0) / F::new(135.0) * t5208;
    let t10424 = t3345 * t572;
    let t10425 = t10424 * t418;
    let t10426 = t1821 * t10425;
    (t10417, t10421, t10423, t10426)
}
