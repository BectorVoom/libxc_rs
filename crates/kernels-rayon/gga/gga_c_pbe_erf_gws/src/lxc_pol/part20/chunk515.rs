//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 515/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk515(t1: f64, t959: f64, t467: f64, t1220: f64, t1278: f64, t1288: f64, t1296: f64, t1328: f64, t1335: f64, t1338: f64, t1426: f64, t1431: f64, t1450: f64, t2064: f64, t2449: f64, t2456: f64, t2476: f64) -> (f64, f64, f64, f64) {
    let t2506 = t959 * t1;
    let t2507 = t2506 * t467;
    let t2508 = 0.18311555036753159941e-3_f64 * t2507;
    let t2509 = t1220 + t1328 + t1335 - t1338 + t1426 - t2449 + t1450 - t1278 + t1288 + t1296 - t2456 + t2476 - t2064 - t2508 - t1431;
    (t2506, t2507, t2508, t2509)
}
