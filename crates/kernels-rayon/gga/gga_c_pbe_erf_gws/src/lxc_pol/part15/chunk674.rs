//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 674/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk674(t2494: f64, t904: f64, t933: f64, t1158: f64, t2323: f64, t1150: f64, t2319: f64, t2204: f64, t2320: f64, t2324: f64, t2336: f64, t3174: f64, t3175: f64, t3188: f64, t3197: f64, t929: f64) -> (f64, f64) {
    let t3268 = t933 * t904 * t2494;
    let t3271 = t2323 * t1158;
    let t3274 = t2319 * t1150;
    let t3277 = t2204 - t3175 - t929 * t3268 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t3271 - t3174 + t3188 - t3197 - 7.0_f64 / 2304.0_f64 * t2320 - 7.0_f64 / 2304.0_f64 * t3274 + 7.0_f64 / 1152.0_f64 * t2324 + t2336;
    (t3268, t3277)
}
