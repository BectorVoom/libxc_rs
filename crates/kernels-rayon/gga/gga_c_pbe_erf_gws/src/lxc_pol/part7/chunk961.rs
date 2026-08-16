//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 961/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk961(t17802: f64, t17771: f64, t17773: f64, t17778: f64, t17783: f64, t17785: f64, t17790: f64, t17794: f64, t17796: f64, t17798: f64, t17800: f64, t211: f64, t5112: f64, t582: f64) -> (f64, f64, f64) {
    let t17803 = 16.0_f64 / 45.0_f64 * t17802;
    let t17804 = -t17771 - t17773 - t17778 - t17783 + t17785 + t17790 + t17794 - t17796 - t17798 + t17800 - t17803;
    let t17806 = t211 * t582 * t5112;
    (t17803, t17804, t17806)
}
