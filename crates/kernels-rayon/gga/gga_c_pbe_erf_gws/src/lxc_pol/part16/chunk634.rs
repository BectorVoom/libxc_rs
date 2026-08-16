//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 634/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk634(t1161: f64, t2376: f64, t830: f64, t829: f64, t1105: f64, t831: f64, t2370: f64, t1114: f64, t2358: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3045 = t2376 * t1161;
    let t3046 = t830 * t3045;
    let t3047 = t829 * t3046;
    let t3050 = t831 * t1105;
    let t3051 = t830 * t3050;
    let t3052 = t2370 * t3051;
    let t3055 = t1114 * t2358;
    let t3060 = t1161 * t810;
    (t3045, t3047, t3050, t3052, t3055, t3060)
}
