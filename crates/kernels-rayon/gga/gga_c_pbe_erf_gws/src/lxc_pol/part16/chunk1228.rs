//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1228/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1228(t1161: f64, t13781: f64, t2079: f64, t3972: f64, t9370: f64, t1123: f64, t2313: f64, t50998: f64, t51021: f64, t938: f64, t14767: f64, t2373: f64) -> (f64, f64, f64) {
    let t53065 = t3972 * t13781 * t1161 * t2079 * t9370;
    let t53072 = t50998 * t51021 * t1123 * t2313 * t938;
    let t53126 = t14767 * t2373;
    (t53065, t53072, t53126)
}
