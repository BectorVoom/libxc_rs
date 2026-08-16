//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 451/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk451(t1663: f64, t950: f64, t1022: f64, t626: f64, t572: f64, t995: f64, t331: f64, t641: f64, t1044: f64, t1791: f64, t649: f64, t1032: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2560 = t1663 * t950;
    let t2570 = t1022 * t626;
    let t2579 = t995 * t572;
    let t2591 = t331 * t641;
    let t2601 = t1791 * t1044;
    let t2607 = t649 * t1022;
    let t2612 = t1032 * t586;
    (t2560, t2570, t2579, t2591, t2601, t2607, t2612)
}
