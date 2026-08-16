//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 948/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk948(t1416: f64, t1620: f64, t1809: f64, t4901: f64, t1413: f64, t1642: f64, t1733: f64, t2677: f64, t1697: f64, t4367: f64, t5002: f64, t617: f64) -> (f64, f64, f64, f64) {
    let t17591 = 16.0_f64 / 15.0_f64 * t1620 * t1809 * t4901 * t1416;
    let t17596 = 16.0_f64 / 9.0_f64 * t1620 * t2677 * t1733 * t1642 * t1413;
    let t17601 = 32.0_f64 / 15.0_f64 * t1620 * t1809 * t1733 * t1697 * t1413;
    let t17606 = 64.0_f64 / 9.0_f64 * t1620 * t2677 * t617 * t5002 * t4367;
    (t17591, t17596, t17601, t17606)
}
