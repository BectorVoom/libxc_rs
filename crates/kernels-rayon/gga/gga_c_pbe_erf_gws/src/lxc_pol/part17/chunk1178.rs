//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1178/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1178(t2079: f64, t3037: f64, t858: f64, t892: f64, t1114: f64, t20112: f64, t12275: f64, t13763: f64, t1143: f64, t6126: f64, t1144: f64, t2416: f64, t3199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29287 = t2079 * t3037;
    let t29751 = t858 * t892;
    let t29775 = t1114 * t20112;
    let t30098 = t12275 * t13763;
    let t34963 = t1143 * t6126;
    let t35566 = t858 * t1144;
    let t36129 = t3199 * t2416;
    (t29287, t29751, t29775, t30098, t34963, t35566, t36129)
}
