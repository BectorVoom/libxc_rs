//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1226/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1226(t13972: f64, t14443: f64, t1123: f64, t52033: f64, t833: f64, t850: f64, t14423: f64, t14682: f64, t3989: f64, t6360: f64, t50998: f64, t51066: f64, t9650: f64) -> (f64, f64, f64, f64) {
    let t53011 = t13972 * t14443;
    let t53015 = t850 * t1123 * t52033 * t833;
    let t53019 = t3989 * t14682 * t14423 * t6360;
    let t53038 = t50998 * t51066 * t9650;
    (t53011, t53015, t53019, t53038)
}
