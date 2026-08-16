//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 604/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk604(t3189: f64, t858: f64, t2210: f64, t884: f64, t2494: f64, t886: f64, t2204: f64, t3170: f64, t3174: f64, t3175: f64, t3176: f64, t3177: f64, t3182: f64, t3186: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3190 = t858 * t3189;
    let t3191 = t2210 * t3190;
    let t3193 = t884 * t3191 / 16.0_f64;
    let t3194 = t858 * t2494;
    let t3195 = t886 * t3194;
    let t3197 = t884 * t3195 / 48.0_f64;
    let t3198 = -t3170 - t3174 - t3175 + t3176 + t2204 + t3177 - t3182 + t3186 + t3188 + t3193 - t3197;
    (t3190, t3191, t3193, t3195, t3197, t3198)
}
