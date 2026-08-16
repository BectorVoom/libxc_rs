//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 933/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk933(t3530: f64, t5283: f64, t587: f64, t2598: f64, t7527: f64, t3535: f64, t7136: f64, t5312: f64, t2635: f64, t2784: f64, t1885: f64, t1820: f64) -> (f64, f64, f64, f64, f64) {
    let t10472 = t5283 * t3530;
    let t10473 = t587 * t10472;
    let t10474 = 8.0_f64 / 81.0_f64 * t10473;
    let t10476 = 8.0_f64 / 15.0_f64 * t7527 * t2598;
    let t10478 = 8.0_f64 / 15.0_f64 * t7136 * t3535;
    let t10480 = 8.0_f64 / 15.0_f64 * t5312 * t3535;
    let t10481 = t2635 * t2784;
    let t10482 = t1885 * t10481;
    let t10484 = 8.0_f64 / 15.0_f64 * t1820 * t10482;
    (t10474, t10476, t10478, t10480, t10484)
}
