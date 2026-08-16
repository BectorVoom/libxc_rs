//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1247/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1247(t14765: f64, t3074: f64, t4395: f64, t1161: f64, t874: f64, t3102: f64, t859: f64, t2370: f64, t36199: f64, t830: f64, t9296: f64, t14692: f64, t3979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54580 = t3074 * t4395 * t14765;
    let t54590 = t1161 * t874;
    let t54595 = t859 * t3102;
    let t54598 = t36199 * t2370;
    let t54599 = t830 * t9296;
    let t54616 = t3979 * t14692;
    (t54580, t54590, t54595, t54598, t54599, t54616)
}
