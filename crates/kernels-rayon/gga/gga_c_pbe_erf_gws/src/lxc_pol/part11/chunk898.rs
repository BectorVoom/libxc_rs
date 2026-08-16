//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 898/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk898(t5385: f64, t720: f64, t1365: f64, t252: f64, t254: f64, t16569: f64, t5560: f64, t24: f64, t247: f64, t5426: f64, t712: f64, t248: f64, t256: f64, t7236: f64, t7271: f64) -> (f64, f64, f64, f64, f64) {
    let t18240 = 32.0_f64 / 81.0_f64 * t720 * t5385;
    let t18243 = 56.0_f64 / 243.0_f64 * t252 * t254 * t1365;
    let t18245 = 0.80823369382716049381e-2_f64 * t16569 * t5560;
    let t18261 = 0.24311111111111111111e0_f64 * t712 * t24 * t247 * t5426;
    let t18267 = t248 * (-0.33530864197530864197e0_f64 * t7271 + 0.18360493827160493828e1_f64 * t7236) * t256 / 3.0_f64;
    (t18240, t18243, t18245, t18261, t18267)
}
