//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1023/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1023(t3854: f64, t6: f64, t2171: f64, t2345: f64, t11459: f64, t3139: f64, t875: f64, t2168: f64, t2494: f64, t343: f64, t2170: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11464 = t6 * t3854;
    let t11466 = t2345 * t11464 * t2171;
    let t11470 = t3139 * t11459 * t875;
    let t11472 = t2168 * t11470 / 96.0_f64;
    let t11473 = t343 * t2494;
    let t11475 = t2170 * t3131 * t11473;
    (t11464, t11466, t11470, t11472, t11473, t11475)
}
