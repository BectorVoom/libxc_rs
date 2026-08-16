//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 650/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk650(t247: f64, t3379: f64, t251: f64, t1619: f64, t1675: f64, t2536: f64, t256: f64, t2651: f64, t2655: f64, t2657: f64, t3388: f64, t3389: f64, t3394: f64, t3401: f64, t3405: f64) -> (f64, f64, f64) {
    let t3583 = t3379 * t247;
    let t3584 = t3583 * t251;
    let t3591 = t1619 + t3584 * t256 / 3.0_f64 - 4.0_f64 / 45.0_f64 * t2536 + t3388 + t3389 + t3394 + 2.0_f64 / 3.0_f64 * t2651 + 0.12155555555555555555e0_f64 * t2655 + 4.0_f64 / 9.0_f64 * t2657 - t1675 + t3401 + t3405;
    (t3583, t3584, t3591)
}
