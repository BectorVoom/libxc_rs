//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 337/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk337(t1275: f64, t1293: f64, t1291: f64, t155: f64, t449: f64, t1215: f64, t75: f64, t1216: f64, t456: f64) -> (f64, f64, f64, f64, f64) {
    let t1294 = t1275 * t1293;
    let t1295 = t1291 * t1294;
    let t1296 = 0.16081824322151104822e2_f64 * t1295;
    let t1300 = t155 * t449;
    let t1304 = t75 * t1215;
    let t1305 = t1216 * t456;
    (t1294, t1296, t1300, t1304, t1305)
}
