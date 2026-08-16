//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 425/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk425(t1504: f64, t1563: f64, t127: f64, t1511: f64, t1517: f64, t1519: f64, t1522: f64, t1533: f64, t1536: f64, t1540: f64, t1542: f64, t1545: f64, t1549: f64, t1555: f64, t1558: f64, t1561: f64, t496: f64, t506: f64) -> f64 {
    let t1564 = t1563 * t1504;
    let t1570 = -t1511 + t1517 + t1519 + t1522 - t1536 + t1540 + t1542 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t496 * t1545 - t496 * t1549 / 2.0_f64 + t1555 + 0.146904e1_f64 * t1558 + t1561 + 0.587616e1_f64 * t127 * t1564 - 0.146904e1_f64 * t127 * t506 * t1533;
    t1570
}
