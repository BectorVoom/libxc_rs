//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 387/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk387(t11: f64, t19: f64, t1237: f64, t1240: f64, t398: f64, t21: f64, t703: f64) -> (f64, f64, f64, f64) {
    let t1245 = 1.0_f64/f64::sqrt(t11);
    let t1246 = t1245 * t19;
    let t1247 = t1246 * t1237;
    let t1249 = t398 * t1240;
    let t1251 = t21 * t703;
    (t1246, t1247, t1249, t1251)
}
