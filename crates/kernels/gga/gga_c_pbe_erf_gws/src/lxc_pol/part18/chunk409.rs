//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 409/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk409<F: Float>(t1479: F, t1480: F, t751: F, t755: F, t759: F, t285: F, t535: F, t545: F, t281: F, t1368: F, t147: F, t520: F, t524: F, t142: F, t100: F, t95: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1482 = 0.18218576931715098443e-4 * t1479 * t1480;
    let t1483 = t751 * t755;
    let t1486 = 0.39914113367515363646e-1 * t751 * t759;
    let t1492 = t535 * t545 * t285;
    let t1493 = t281 * t1492;
    let t1497 = t147 * t1368 * t285;
    let t1499 = 0.11974234010254609094e-1 * t281 * t1497;
    let t1500 = t524 * t520;
    let t1501 = t1500 * t142;
    let t1503 = t95 * t100;
    (t1482, t1483, t1486, t1492, t1493, t1497, t1499, t1500, t1501, t1503)
}
