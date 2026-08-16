//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1186/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1186(t13808: f64, t13877: f64, t2403: f64, t8599: f64, t2332: f64, t864: f64, t899: f64, t907: f64, t13806: f64, t915: f64, t2276: f64, t2281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51153 = t13808 * t13877;
    let t51179 = t8599 * t2403;
    let t51200 = t899 * t864 * t2332;
    let t51201 = t51200 * t907;
    let t51213 = t13806 * t915;
    let t51214 = t2276 * t51213;
    let t51215 = t51214 * t2281;
    (t51153, t51179, t51200, t51201, t51213, t51214, t51215)
}
