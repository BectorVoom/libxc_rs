//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 381/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk381(t1215: f64, t1216: f64, t456: f64, t470: f64, t155: f64, t434: f64, t433: f64, t67: f64, t62: f64, t440: f64, t441: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1218 = t1215 * t1216 * t456;
    let t1219 = t470 * t1218;
    let t1220 = 0.11696446794910408142e1_f64 * t1219;
    let t1224 = t155 * t434;
    let t1228 = t433 * t67;
    let t1229 = 1.0_f64 / t1228;
    let t1230 = t62 * t1229;
    let t1231 = t440 * t440;
    let t1232 = t1231 * t441;
    let t1235 = 1.0_f64 / t126;
    (t1218, t1220, t1224, t1229, t1230, t1231, t1232, t1235)
}
