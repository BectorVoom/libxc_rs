//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 933/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk933(t375: f64, t6125: f64, t4422: f64, t828: f64, t2331: f64, t362: f64, t915: f64, t2250: f64, t4395: f64, t6670: f64, t356: f64, t358: f64, t6552: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20173 = 1.0_f64 / t6125 / t375;
    let t20189 = t4422 * t828;
    let t20269 = t362 * t2331;
    let t20270 = t20269 * t915;
    let t20271 = t2250 * t20270;
    let t20281 = t4395 * t6670;
    let t20303 = t356 * t358 * t6552;
    (t20173, t20189, t20269, t20270, t20271, t20281, t20303)
}
