//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 579/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk579(t2179: f64, t4724: f64, t144: f64, t167: f64, t2185: f64, t4668: f64, t1017: f64, t1053: f64, t574: f64, t605: f64, t1060: f64, t569: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4725 = t2179 * t4724;
    let t4726 = t144 * t4725;
    let t4730 = t2185 * t167 * t4668;
    let t4733 = t1017 * t1053;
    let t4735 = t574 * t605 * t4733;
    let t4739 = t569 * t1060 * t925;
    (t4725, t4726, t4730, t4733, t4735, t4739)
}
