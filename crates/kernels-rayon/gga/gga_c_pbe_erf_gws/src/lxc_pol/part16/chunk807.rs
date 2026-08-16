//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 807/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk807(t2157: f64, t343: f64, t2306: f64, t346: f64, t2382: f64, t2074: f64, t337: f64, t5: f64, t2147: f64, t2189: f64, t2251: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6241 = t2157 * t343;
    let t6252 = t2306 * t346;
    let t6253 = t2382 * t6252;
    let t6257 = t337 * t5 * t2074;
    let t6258 = t2147 * t6257;
    let t6269 = t5 * t2189;
    let t6274 = t2251 * t933;
    (t6241, t6253, t6257, t6258, t6269, t6274)
}
