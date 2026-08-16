//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 798/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk798(t2157: f64, t343: f64, t2306: f64, t346: f64, t2251: f64, t933: f64, t2250: f64, t810: f64, t2365: f64, t885: f64, t2149: f64, t4395: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6241 = t2157 * t343;
    let t6252 = t2306 * t346;
    let t6274 = t2251 * t933;
    let t6275 = t2250 * t6274;
    let t6287 = t2157 * t810;
    let t6331 = t2365 * t885;
    let t6332 = t6331 * t2149;
    let t6335 = t4395 * t346;
    (t6241, t6252, t6275, t6287, t6331, t6332, t6335)
}
