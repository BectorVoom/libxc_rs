//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 784/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk784(t496: f64, t5787: f64, t120: f64, t1508: f64, t19: f64, t5763: f64, t1: f64, t1563: f64, t501: f64, t119: f64, t1504: f64, t155: f64) -> (f64, f64, f64, f64) {
    let t5788 = t496 * t5787;
    let t5795 = t1508 * t120 * t19;
    let t5796 = t5795 * t5763;
    let t5803 = t501 * t1563 * t1;
    let t5805 = t119 * t155 * t1504;
    (t5788, t5796, t5803, t5805)
}
