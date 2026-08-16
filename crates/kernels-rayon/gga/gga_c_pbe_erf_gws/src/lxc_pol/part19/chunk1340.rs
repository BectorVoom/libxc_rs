//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1340/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1340(t14682: f64, t3140: f64, t3989: f64, t57321: f64, t13815: f64, t3781: f64, t833: f64, t850: f64, t11624: f64, t13917: f64, t51066: f64, t2249: f64, t56296: f64) -> (f64, f64, f64, f64) {
    let t57574 = t3989 * t14682 * t57321 * t3140;
    let t57578 = t850 * t3781 * t13815 * t833;
    let t57584 = t13917 * t51066 * t11624;
    let t57591 = t2249 * t56296;
    (t57574, t57578, t57584, t57591)
}
