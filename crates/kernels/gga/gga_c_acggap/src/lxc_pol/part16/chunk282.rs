//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 282/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk282<F: Float>(t43: F, t1240: F, t322: F, t381: F, t452: F, t879: F, t180: F, t939: F, t394: F, t441: F, t702: F, t705: F, t474: F, t817: F, t292: F, t34: F, t234: F, t821: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t1241 = t1240 * t322;
    let t1242 = t381 * t1241;
    let t1244 = t452 * t879;
    let t1246 = 0.65854491829355115987e0 * t381 * t1244;
    let t1247 = t939 * t180;
    let t1251 = t394 * t441;
    let t1279 = 4.0 * t702;
    let t1280 = 0.18311447306006545054e-3 * t705;
    let t1281 = t817 * t474;
    let t1284 = t292 * t34;
    let t1288 = piecewise3(t44, 0.0, -2.0 / 9.0 * t1281 * t234 + 4.0 / 3.0 * t1284 * t821);
    (t1241, t1242, t1244, t1246, t1247, t1251, t1279, t1280, t1281, t1288)
}
