//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 835/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk835(t7983: f64, t1333: f64, t960: f64, t4753: f64, t1326: f64, t959: f64, t40: f64, t1444: f64, t2506: f64, t2513: f64, t409: f64, t2515: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7984 = 8.0_f64 * t7983;
    let t7986 = t1333 * t960;
    let t7994 = 12.0_f64 * t4753;
    let t7996 = t959 * t1326;
    let t7997 = t40 * t7996;
    let t8004 = t2506 * t1444;
    let t8010 = t409 * t2513;
    let t8011 = 8.0_f64 * t8010;
    let t8012 = t414 * t2515;
    (t7984, t7986, t7994, t7997, t8004, t8011, t8012)
}
