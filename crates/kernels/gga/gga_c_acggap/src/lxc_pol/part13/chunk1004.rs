//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1004/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1004<F: Float>(t7413: F, t7835: F, t8480: F, t30219: F, t8446: F, t1439: F, t30148: F, t30154: F, t7842: F, t31363: F, t31374: F, t31377: F, t31381: F, t31390: F, t31392: F, t31407: F, t35545: F, t35550: F, t35553: F, t35557: F, t35560: F, t35562: F, t35563: F, t35564: F) -> (F,) {
    let t35567 = t7413 * t8480 * t7835;
    let t35569 = t30219 * t8446;
    let t35570 = 0.31448092289604152068e-2 * t35569;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    let t35574 = 0.31448092289604152068e-2 * t35573;
    let t35575 = 0.17149607247227894789e-2 * t35545 - t35550 + t35553 - t35557 - 0.15724046144802076034e-2 * t31363 + 0.16809375e0 * t31374 + 35.0 / 432.0 * t35560 - t31377 - t31381 + t35562 + t35563 + t31390 - t31392 - t31407 + 0.13719685797782315831e-1 * t35564 + 0.21437009059034868486e-3 * t35567 + t35570 - t35574;
    (t35575,)
}
