//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1189/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1189<F: Float>(t100410: F, t100427: F, t116473: F, t116477: F, t116481: F, t116485: F, t116487: F, t116490: F, t116493: F, t116495: F, t116500: F, t116503: F, t100431: F, t100478: F, t100480: F, t116508: F, t116512: F, t116515: F, t116518: F, t116521: F, t116526: F, t116530: F, t116534: F, t116537: F) -> (F, F) {
    let t117123 = t116473 / 9.0 + 2.0 / 27.0 * t116477 - 4.0 * t116481 - t116485 + 2.0 / 27.0 * t116487 + t116490 / 3.0 - t100410 - 2.0 / 9.0 * t116493 - 2.0 / 27.0 * t116495 + t116500 / 9.0 - t116503 / 9.0 - 4.0 / 27.0 * t100427;
    let t117133 = t100431 + t116508 / 9.0 + t116512 / 9.0 + 2.0 / 3.0 * t116515 + 4.0 / 9.0 * t116518 - 4.0 / 27.0 * t116521 - t116526 / 36.0 - t116530 / 36.0 - 2.0 / 9.0 * t116534 + 2.0 / 3.0 * t116537 + t100478 + t100480;
    (t117123, t117133)
}
