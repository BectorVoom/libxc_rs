//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 743/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk743(t13: f64, t4508: f64, t1275: f64, t403: f64, t1293: f64, t14: f64, t25: f64, t2: f64, t39: f64, t784: f64, t799: f64, t1236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4509 = t13 * t4508;
    let t4510 = t1275 * t403;
    let t4511 = t4510 * t1293;
    let t4512 = t4509 * t4511;
    let t4513 = 0.96490945932906628932e2_f64 * t4512;
    let t4516 = 1.0_f64 / t14 / t25 / 4.0_f64;
    let t4517 = t4516 * t2;
    let t4518 = t4517 * t39;
    let t4520 = t799 * t784;
    let t4521 = t1236 * t4520;
    (t4510, t4513, t4516, t4518, t4520, t4521)
}
