//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1244/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1244(t14058: f64, t3268: f64, t1140: f64, t14083: f64, t3190: f64, t3206: f64, t2407: f64, t26623: f64, t858: f64, t3195: f64, t4033: f64, t4171: f64, t51407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54354 = t14058 * t3268;
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54373 = t2407 * t858 * t26623;
    let t54377 = t4033 * t3195;
    let t54381 = t51407 * t4171;
    (t54354, t54356, t54359, t54373, t54377, t54381)
}
