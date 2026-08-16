//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 634/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk634(t995: f64, t181: f64, t184: f64, t199: f64, t2570: f64, t954: f64, t1809: f64, t1620: f64, t1027: f64, t1044: f64, t1815: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3397 = t995 * t995;
    let t3398 = t3397 * t181;
    let t3399 = t3398 * t184;
    let t3401 = 4.0_f64 / 15.0_f64 * t3399 * t199;
    let t3402 = t2570 * t954;
    let t3403 = t1809 * t3402;
    let t3405 = 16.0_f64 / 45.0_f64 * t1620 * t3403;
    let t3406 = t1027 * t1044;
    let t3407 = t1815 * t3406;
    let t3409 = 8.0_f64 / 45.0_f64 * t639 * t3407;
    (t3397, t3398, t3399, t3401, t3402, t3403, t3405, t3406, t3407, t3409)
}
