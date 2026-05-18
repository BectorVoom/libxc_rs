//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1027/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1027<F: Float>(t3078: F, t9246: F, t3077: F, t3103: F, t840: F, t3307: F, t338: F, t892: F, t376: F, t8574: F, t353: F, t9169: F) -> (F, F, F, F, F, F) {
    let t9247 = t9246 * t3078;
    let t9249 = F::new(7.0) / F::new(144.0) * t3077 * t9247;
    let t9253 = F::new(7.0) / F::new(144.0) * t840 * t3103;
    let t9255 = t338 * t892 * t3307;
    let t9258 = t376 * t8574;
    let t9260 = t338 * t353 * t9258;
    let t9263 = param_a_c * t9169;
    (t9249, t9253, t9255, t9258, t9260, t9263)
}
