//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 655/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk655(t142: f64, t981: f64, t2031: f64, t159: f64, t285: f64, t3379: f64, t1523: f64, t3342: f64, t3346: f64, t476: f64, t1528: f64, t3351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3619 = t142 * t981;
    let t3620 = t2031 * t3619;
    let t3626 = t3379 * t159 * t285;
    let t3629 = t1523 * t3342;
    let t3631 = t476 * t3346;
    let t3633 = t1528 * t3351;
    (t3619, t3620, t3626, t3629, t3631, t3633)
}
