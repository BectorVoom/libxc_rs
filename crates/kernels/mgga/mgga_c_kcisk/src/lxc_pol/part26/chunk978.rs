//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 978/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk978<F: Float>(t338: F, t25432: F, t6183: F, t25437: F, t6175: F, t442: F, t8048: F, t1056: F, t13472: F, t8054: F, t3937: F, t25: F, t8055: F, t1309: F, t25906: F, t1320: F, t1310: F) -> (F, F, F, F, F, F, F) {
    let t400 = 0.0 < t338;
    let t26447 = t6183 * t25432;
    let t26450 = t6175 * t25437;
    let t26453 = t8048 * t442;
    let t26454 = t26453 * t1056;
    let t26455 = t13472 * t26454;
    let t26458 = t8054 * t442;
    let t26459 = t26458 * t1056;
    let t26460 = t3937 * t26459;
    let t26470 = t25 * t8055;
    let t26471 = t1309 * t26470;
    let t26476 = piecewise3(t400, t25906, -t25906);
    let t26477 = t1320 * t26476;
    let t26478 = t1310 * t26477;
    (t26447, t26450, t26455, t26460, t26471, t26476, t26478)
}
