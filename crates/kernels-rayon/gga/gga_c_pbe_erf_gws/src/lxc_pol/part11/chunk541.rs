//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 541/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk541(t133: f64, t1511: f64, t1519: f64, t1583: f64, t2909: f64, t3648: f64, t3651: f64, t3654: f64, t3657: f64, t3661: f64) -> f64 {
    let t3683 = -t1511 + t3648 + t1519 + t3651 - t3654 + t1583 + 0.11495033333333333333e1_f64 * t2909 + 0.5172765e1_f64 * t133 * t3657 - 0.1724255e1_f64 * t133 * t3661;
    t3683
}
