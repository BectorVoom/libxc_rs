//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 837/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk837(t1336: f64, t960: f64, t1396: f64, t2840: f64, t1392: f64, t1: f64, t2474: f64, t467: f64, t1218: f64, t75: f64, t472: f64, t4853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8014 = t1336 * t960;
    let t8016 = t2840 * t1396;
    let t8018 = t2840 * t1392;
    let t8020 = t2474 * t1;
    let t8021 = t8020 * t467;
    let t8022 = 0.36623110073506319882e-3_f64 * t8021;
    let t8023 = t2840 * t1218;
    let t8029 = t2474 * t75;
    let t8030 = t8029 * t472;
    let t8031 = 0.11696446794910408142e1_f64 * t8030;
    let t8033 = 32.0_f64 * t4853;
    (t8014, t8016, t8018, t8022, t8023, t8031, t8033)
}
