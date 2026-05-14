//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1160/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1160<F: Float>(t1882: F, t29630: F, t29569: F, t358: F, t1564: F, t363: F, t446: F, t1317: F, t29702: F, t376: F, t100427: F, t102144: F, t116473: F, t116477: F, t116481: F, t116485: F, t116488: F, t116490: F, t116493: F) -> (F, F, F, F) {
    let t116495 = t1882 * t29630;
    let t116496 = 2.0 / 9.0 * t116495;
    let t116497 = t29569 * t358;
    let t116500 = t446 * t1564 * t116497 * t363;
    let t116503 = t1317 * t376 * t29702;
    let t116504 = t116503 / 3.0;
    let t116506 = t116473 / 3.0 + 2.0 / 9.0 * t116477 - 12.0 * t116481 - 3.0 * t116485 + t116488 + t116490 - t102144 - 2.0 / 3.0 * t116493 - t116496 + t116500 / 3.0 - t116504 - 4.0 / 9.0 * t100427;
    (t116495, t116500, t116503, t116506)
}
