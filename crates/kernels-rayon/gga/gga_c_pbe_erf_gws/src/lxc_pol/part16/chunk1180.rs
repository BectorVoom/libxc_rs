//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1180/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1180(t12275: f64, t13763: f64, t1143: f64, t6126: f64, t1144: f64, t858: f64, t2416: f64, t3199: f64, t326: f64, t825: f64, t6148: f64, t3067: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30098 = t12275 * t13763;
    let t34963 = t1143 * t6126;
    let t35566 = t858 * t1144;
    let t36129 = t3199 * t2416;
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    (t30098, t34963, t35566, t36129, t36199, t36200, t36201)
}
