//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1198/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1198<F: Float>(t101676: F, t101681: F, t101688: F, t101690: F, t101691: F, t101696: F, t93453: F, t93455: F, t93458: F, t93474: F, t93477: F, t93480: F, t1564: F, t1580: F, t25955: F, t446: F) -> (F, F) {
    let t101698 = t93453 + t93455 / 9.0 - 8.0 / 27.0 * t93458 - 2.0 / 27.0 * t101676 + 4.0 / 3.0 * t101681 + 16.0 / 27.0 * t93474 - 2.0 / 9.0 * t93477 - 4.0 / 9.0 * t93480 + t101688 + t101690 + 2.0 / 81.0 * t101691 - 2.0 / 9.0 * t101696;
    let t101701 = t446 * t1564 * t25955 * t1580;
    (t101698, t101701)
}
