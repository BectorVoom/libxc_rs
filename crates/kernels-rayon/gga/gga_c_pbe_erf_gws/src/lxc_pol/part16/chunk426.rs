//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 426/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk426(t510: f64, t513: f64, t137: f64, t512: f64, t131: f64, t520: f64, t120: f64, t133: f64, t542: f64, t1541: f64, t1511: f64, t1517: f64, t1519: f64, t1522: f64, t1536: f64, t1545: f64, t1549: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1572 = t510 * t513;
    let t1576 = 1.0_f64 / t512 / t137;
    let t1577 = t131 * t1576;
    let t1578 = t520 * t520;
    let t1583 = 0.38316777777777777777e0_f64 * t133 * t542 * t120;
    let t1584 = t133 * t1541;
    let t1590 = -t1511 + t1517 + t1519 + t1522 - t1536 + t1583 + 0.11495033333333333333e1_f64 * t1584 + 0.5172765e1_f64 * t133 * t1545 - 0.1724255e1_f64 * t133 * t1549;
    (t1572, t1576, t1577, t1578, t1583, t1584, t1590)
}
