//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 413/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk413(t1479: f64, t1480: f64, t751: f64, t755: f64, t759: f64, t285: f64, t535: f64, t545: f64, t281: f64, t1368: f64, t147: f64, t520: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1482 = 0.18218576931715098443e-4_f64 * t1479 * t1480;
    let t1483 = t751 * t755;
    let t1486 = 0.39914113367515363646e-1_f64 * t751 * t759;
    let t1492 = t535 * t545 * t285;
    let t1493 = t281 * t1492;
    let t1497 = t147 * t1368 * t285;
    let t1499 = 0.11974234010254609094e-1_f64 * t281 * t1497;
    let t1500 = t524 * t520;
    (t1482, t1483, t1486, t1492, t1493, t1497, t1499, t1500)
}
