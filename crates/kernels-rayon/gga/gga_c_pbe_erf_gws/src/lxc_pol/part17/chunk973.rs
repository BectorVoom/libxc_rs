//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 973/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk973(t2391: f64, t3039: f64, t2246: f64, t3090: f64, t3094: f64, t3309: f64, t840: f64, t3306: f64, t938: f64, t2409: f64, t3067: f64, t3075: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8634 = t3039 * t2391;
    let t8641 = 7.0_f64 / 72.0_f64 * t2246 * t3090;
    let t8643 = 7.0_f64 / 72.0_f64 * t2246 * t3094;
    let t8646 = 7.0_f64 / 144.0_f64 * t840 * t3309;
    let t8647 = t3306 * t938;
    let t8649 = t2409 * t3067 * t8647;
    let t8652 = t3075 * t331;
    (t8634, t8641, t8643, t8646, t8647, t8649, t8652)
}
