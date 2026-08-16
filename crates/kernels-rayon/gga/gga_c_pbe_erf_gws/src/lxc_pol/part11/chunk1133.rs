//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1133/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1133(t10419: f64, t3564: f64, t12711: f64, t2741: f64, t10743: f64, t186: f64, t220: f64, t47638: f64, t616: f64, t3451: f64, t40402: f64, t10969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48067 = 8.0_f64 / 5.0_f64 * t10419 * t3564;
    let t48069 = 16.0_f64 / 15.0_f64 * t2741 * t12711;
    let t48071 = 16.0_f64 / 15.0_f64 * t10743 * t12711;
    let t48076 = -4.0_f64 / 15.0_f64 * t616 * t186 * t220 * t47638;
    let t48078 = 16.0_f64 / 5.0_f64 * t40402 * t3451;
    let t48080 = 8.0_f64 / 5.0_f64 * t10969 * t3451;
    (t48067, t48069, t48071, t48076, t48078, t48080)
}
