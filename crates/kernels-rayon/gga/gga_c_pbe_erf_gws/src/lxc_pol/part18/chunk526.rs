//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 526/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk526(t1022: f64, t649: f64, t661: f64, t1621: f64, t1620: f64, t1032: f64, t586: f64) -> (f64, f64, f64, f64, f64) {
    let t2607 = t649 * t1022;
    let t2608 = t2607 * t661;
    let t2609 = t1621 * t2608;
    let t2611 = 4.0_f64 / 15.0_f64 * t1620 * t2609;
    let t2612 = t1032 * t586;
    (t2607, t2608, t2609, t2611, t2612)
}
