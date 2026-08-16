//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 372/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk372(t1479: f64, t1480: f64, t751: f64, t755: f64, t759: f64, t1354: f64, t159: f64, t285: f64, t535: f64, t545: f64, t281: f64, t1342: f64, t1345: f64, t1349: f64, t1355: f64, t1360: f64, t1386: f64, t1388: f64, t1389: f64, t145: f64, t1452: f64, t1459: f64, t1463: f64, t1467: f64, t1471: f64, t1475: f64, t169: f64, t242: f64, t296: f64) -> (f64, f64, f64) {
    let t1482 = 0.18218576931715098443e-4_f64 * t1479 * t1480;
    let t1483 = t751 * t755;
    let t1486 = 0.39914113367515363646e-1_f64 * t751 * t759;
    let t1488 = t1354 * t159 * t285;
    let t1492 = t535 * t545 * t285;
    let t1493 = t281 * t1492;
    let t1495 = (-t1342 + 0.1061188859155979109e0_f64 * t1345 + t1349 - 0.31835665774679373271e-1_f64 * t169 * t1355 * t242 - 0.63671331549358746542e-1_f64 * t1360 - t1386 + t1388 - 0.2133002709687175212e0_f64 * t1389 + 0.533250677421793803e-1_f64 * t145 * t1452) * t296 - 0.58113483035773838734e-3_f64 * t1459 - t1463 + t1467 + t1471 - t1475 - t1482 + 0.39914113367515363646e-1_f64 * t1483 + t1486 - 0.11974234010254609094e-1_f64 * t281 * t1488 - 0.23948468020509218188e-1_f64 * t1493;
    (t1488, t1492, t1495)
}
