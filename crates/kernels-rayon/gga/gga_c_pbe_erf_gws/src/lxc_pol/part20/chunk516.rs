//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 516/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk516(t1432: f64, t1435: f64, t2093: f64, t2474: f64, t87: f64, t40: f64, t460: f64, t959: f64, t1400: f64, t145: f64, t991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2510 = 4.0_f64 * t1432;
    let t2511 = 0.18311555036753159941e-3_f64 * t1435;
    let t2512 = 0.41076328840066666668e0_f64 * t2093;
    let t2513 = t2474 * t87;
    let t2514 = t40 * t2513;
    let t2515 = t959 * t460;
    let t2516 = t40 * t2515;
    let t2517 = 0.58482233974552040708e0_f64 * t1400;
    let t2519 = t145 * t991;
    (t2510, t2511, t2512, t2513, t2514, t2515, t2516, t2517, t2519)
}
