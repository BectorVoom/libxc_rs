//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1181/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1181(t3875: f64, t4049: f64, t11785: f64, t3139: f64, t14101: f64, t1184: f64, t3799: f64, t3867: f64, t3805: f64, t4023: f64, t14031: f64, t3765: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15253 = t4049 * t3875;
    let t15255 = t3139 * t11785;
    let t15256 = t14101 * t15255;
    let t15258 = t1184 * t3799;
    let t15260 = t1184 * t3867;
    let t15262 = t3805 * t4023;
    let t15264 = t14031 * t3765;
    (t15253, t15255, t15256, t15258, t15260, t15262, t15264)
}
