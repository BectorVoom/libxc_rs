//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1263/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1263(t2370: f64, t36199: f64, t830: f64, t9296: f64, t14692: f64, t3979: f64, t4135: f64, t51966: f64, t2242: f64, t4185: f64, t1146: f64, t13987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54598 = t36199 * t2370;
    let t54599 = t830 * t9296;
    let t54616 = t3979 * t14692;
    let t54617 = 7.0_f64 / 2304.0_f64 * t54616;
    let t54621 = t51966 * t4135;
    let t54639 = t2242 * t4185;
    let t54641 = t13987 * t1146;
    (t54598, t54599, t54617, t54621, t54639, t54641)
}
