//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 383/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk383(t120: f64, t133: f64, t542: f64, t1541: f64, t1511: f64, t1517: f64, t1519: f64, t1522: f64, t1536: f64, t1545: f64, t1549: f64) -> f64 {
    let t1583 = 0.38316777777777777777e0_f64 * t133 * t542 * t120;
    let t1584 = t133 * t1541;
    let t1590 = -t1511 + t1517 + t1519 + t1522 - t1536 + t1583 + 0.11495033333333333333e1_f64 * t1584 + 0.5172765e1_f64 * t133 * t1545 - 0.1724255e1_f64 * t133 * t1549;
    t1590
}
