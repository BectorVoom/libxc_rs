//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 224/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk224<F: Float>(t623: F, t190: F, t212: F, t601: F, t205: F, t191: F) -> (F, F, F, F) {
    let t651 = F::new(0.35991666666666666667e-1) * t623;
    let t655 = F::new(0.66666666666666666667e-2) * t190 * t601 * t212;
    let t656 = F::new(1.0) / t205;
    let t657 = t191 * t656;
    (t651, t655, t656, t657)
}
