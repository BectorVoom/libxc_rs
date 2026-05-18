//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 977/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk977<F: Float>(t2366: F, t3039: F, t833: F, t2367: F, t3047: F, t3200: F, t338: F, t939: F, t1162: F, t814: F, t353: F, t859: F) -> (F, F, F, F, F) {
    let t8669 = t3039 * t2366;
    let t8671 = F::new(7.0) / F::new(144.0) * t8669 * t833;
    let t8677 = F::new(7.0) / F::new(144.0) * t2367 * t3047;
    let t8685 = t338 * t3200 * t939;
    let t8688 = t1162 * t814;
    let t8689 = t353 * t8688;
    let t8690 = t859 * t8689;
    (t8669, t8671, t8677, t8685, t8690)
}
