//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 873/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk873(t329: f64, t332: f64, t9239: f64, t838: f64, t857: f64, t3078: f64, t3077: f64, t3103: f64, t840: f64) -> (f64, f64, f64, f64, f64) {
    let t9241 = t329 * t332 * t9239;
    let t9246 = t838 * t857;
    let t9247 = t9246 * t3078;
    let t9249 = 7.0_f64 / 144.0_f64 * t3077 * t9247;
    let t9253 = 7.0_f64 / 144.0_f64 * t840 * t3103;
    let t9270 = t329 * t9246;
    (t9241, t9246, t9249, t9253, t9270)
}
