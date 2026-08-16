//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 584/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk584(t1112: f64, t328: f64, t2306: f64, t3074: f64, t377: f64, t858: f64, t3065: f64, t1114: f64, t2366: f64) -> (f64, f64, f64, f64, f64) {
    let t3075 = t1112 * t328;
    let t3076 = t2306 * t3075;
    let t3077 = t3074 * t3076;
    let t3078 = t858 * t377;
    let t3079 = t3065 * t3078;
    let t3083 = t1114 * t2366;
    (t3075, t3077, t3078, t3079, t3083)
}
