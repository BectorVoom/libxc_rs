//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 510/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk510(t43: f64, t2474: f64, t85: f64, t1523: f64, t950: f64, t418: f64, t34: f64, t476: f64, t532: f64, t1528: f64, t954: f64, t422: f64, t478: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t2475 = t2474 * t85;
    let t2476 = 0.19751789702565206229e-1_f64 * t2475;
    let t2477 = t1523 * t950;
    let t2478 = t2477 * t418;
    let t2480 = t476 * t34;
    let t2481 = t2480 * t532;
    let t2484 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t2478 + 4.0_f64 / 3.0_f64 * t2481);
    let t2485 = t1528 * t954;
    let t2486 = t2485 * t422;
    let t2488 = t478 * t34;
    (t2476, t2477, t2478, t2481, t2484, t2485, t2486, t2488)
}
