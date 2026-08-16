//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 872/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk872<F: Float>(t329: F, t332: F, t9239: F, t838: F, t857: F, t3078: F, t3077: F, t3103: F, t840: F) -> (F, F, F, F, F) {
    let t9241 = t329 * t332 * t9239;
    let t9246 = t838 * t857;
    let t9247 = t9246 * t3078;
    let t9249 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3077 * t9247;
    let t9253 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t3103;
    let t9270 = t329 * t9246;
    (t9241, t9246, t9249, t9253, t9270)
}
