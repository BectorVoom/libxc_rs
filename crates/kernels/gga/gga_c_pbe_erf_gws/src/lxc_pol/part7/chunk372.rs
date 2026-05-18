//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 372/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk372<F: Float>(t1479: F, t1480: F, t751: F, t755: F, t759: F, t1354: F, t159: F, t285: F, t535: F, t545: F, t281: F, t1342: F, t1345: F, t1349: F, t1355: F, t1360: F, t1386: F, t1388: F, t1389: F, t145: F, t1452: F, t1459: F, t1463: F, t1467: F, t1471: F, t1475: F, t169: F, t242: F, t296: F) -> (F, F, F) {
    let t1482 = F::new(0.18218576931715098443e-4) * t1479 * t1480;
    let t1483 = t751 * t755;
    let t1486 = F::new(0.39914113367515363646e-1) * t751 * t759;
    let t1488 = t1354 * t159 * t285;
    let t1492 = t535 * t545 * t285;
    let t1493 = t281 * t1492;
    let t1495 = (-t1342 + F::new(0.1061188859155979109e0) * t1345 + t1349 - F::new(0.31835665774679373271e-1) * t169 * t1355 * t242 - F::new(0.63671331549358746542e-1) * t1360 - t1386 + t1388 - F::new(0.2133002709687175212e0) * t1389 + F::new(0.533250677421793803e-1) * t145 * t1452) * t296 - F::new(0.58113483035773838734e-3) * t1459 - t1463 + t1467 + t1471 - t1475 - t1482 + F::new(0.39914113367515363646e-1) * t1483 + t1486 - F::new(0.11974234010254609094e-1) * t281 * t1488 - F::new(0.23948468020509218188e-1) * t1493;
    (t1488, t1492, t1495)
}
