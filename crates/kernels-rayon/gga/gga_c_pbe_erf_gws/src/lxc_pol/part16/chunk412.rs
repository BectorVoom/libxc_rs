//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 412/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk412(t156: f64, t472: f64, t1447: f64, t1220: f64, t1267: f64, t1271: f64, t1278: f64, t1288: f64, t1296: f64, t1335: f64, t1338: f64, t1440: f64, t1442: f64, t1446: f64) -> (f64, f64, f64) {
    let t1448 = t156 * t472;
    let t1449 = t1447 * t1448;
    let t1450 = 0.10843580882781524214e-1_f64 * t1449;
    let t1451 = t1220 - t1271 - t1278 + t1335 + t1338 - t1440 - t1267 + t1442 + t1288 + t1296 + t1446 + t1450;
    (t1448, t1450, t1451)
}
