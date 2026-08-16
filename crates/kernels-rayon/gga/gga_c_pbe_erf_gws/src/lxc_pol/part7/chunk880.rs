//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 880/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk880(t1620: f64, t5064: f64, t617: f64, t7853: f64, t1630: f64, t1791: f64, t4929: f64, t639: f64, t5109: f64, t642: f64, t422: f64, t5111: f64, t626: f64) -> (f64, f64, f64) {
    let t16796 = 256.0_f64 / 81.0_f64 * t1620 * t7853 * t5064 * t617;
    let t16797 = t1630 * t1791;
    let t16799 = t639 * t16797 * t4929;
    let t16800 = 64.0_f64 / 45.0_f64 * t16799;
    let t16801 = t642 * t5109;
    let t16806 = 32.0_f64 / 15.0_f64 * t639 * t16801 * t5111 * t626 * t422;
    (t16796, t16800, t16806)
}
