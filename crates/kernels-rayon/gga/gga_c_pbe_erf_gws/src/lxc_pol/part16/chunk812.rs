//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 812/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk812(t2298: f64, t322: f64, t2164: f64, t2197: f64, t2192: f64, t2331: f64, t899: f64, t912: f64, t918: f64, t2079: f64, t2105: f64, t4394: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6429 = t322 * t2298;
    let t6445 = t2164 * t2197;
    let t6447 = t2164 * t2192;
    let t6455 = t899 * t912 * t2331;
    let t6456 = t6455 * t918;
    let t6469 = t2079 * param_a_c;
    let t6472 = t4394 * t2105;
    (t6429, t6445, t6447, t6455, t6456, t6469, t6472)
}
