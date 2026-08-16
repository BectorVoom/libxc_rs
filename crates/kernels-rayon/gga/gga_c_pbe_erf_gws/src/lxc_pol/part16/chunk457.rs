//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 457/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk457(t617: f64, t626: f64, t422: f64, t1809: f64, t1620: f64, t642: f64, t649: f64) -> (f64, f64, f64, f64) {
    let t1810 = t617 * t626;
    let t1811 = t1810 * t422;
    let t1812 = t1809 * t1811;
    let t1814 = 16.0_f64 / 45.0_f64 * t1620 * t1812;
    let t1815 = t642 * t649;
    (t1811, t1812, t1814, t1815)
}
