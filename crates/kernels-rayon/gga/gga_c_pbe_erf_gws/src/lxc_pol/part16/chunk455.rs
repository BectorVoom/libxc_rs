//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 455/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk455(t1791: f64, t213: f64, t661: f64, t186: f64, t211: f64, t582: f64, t618: f64, t616: f64, t196: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1792 = t213 * t1791;
    let t1793 = t661 * t661;
    let t1794 = t1792 * t1793;
    let t1795 = t186 * t1794;
    let t1797 = 4.0_f64 / 15.0_f64 * t211 * t1795;
    let t1798 = t582 * t618;
    let t1799 = t616 * t1798;
    let t1800 = 16.0_f64 / 45.0_f64 * t1799;
    let t1802 = 1.0_f64 / t596 / t196;
    (t1793, t1794, t1795, t1797, t1798, t1799, t1800, t1802)
}
