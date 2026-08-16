//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 804/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk804(t745: f64, t810: f64, t2132: f64, t2306: f64, t2382: f64, t2074: f64, t343: f64, t2319: f64, t2339: f64, t1477: f64, t863: f64, t864: f64) -> (f64, f64, f64, f64, f64) {
    let t6211 = t745 * t810;
    let t6216 = t2306 * t2132;
    let t6217 = t2382 * t6216;
    let t6220 = t343 * t2074;
    let t6225 = t2319 * t2339;
    let t6228 = t863 * t864 * t1477;
    (t6211, t6217, t6220, t6225, t6228)
}
