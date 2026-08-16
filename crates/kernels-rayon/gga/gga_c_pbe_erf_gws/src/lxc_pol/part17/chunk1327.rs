//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1327/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1327(t14007: f64, t9478: f64, t14015: f64, t9460: f64, t14570: f64, t6188: f64, t2407: f64, t26623: f64, t858: f64, t2120: f64, t3195: f64, t4033: f64) -> (f64, f64, f64, f64, f64) {
    let t54366 = t14007 * t9478;
    let t54368 = t14015 * t9460;
    let t54370 = t6188 * t14570;
    let t54373 = t2407 * t858 * t26623;
    let t54374 = t2120 * t54373;
    let t54377 = t4033 * t3195;
    (t54366, t54368, t54370, t54374, t54377)
}
