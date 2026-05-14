//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 615/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk615<F: Float>(t13693: F, t2355: F, t13353: F, t13357: F, t13362: F, t13370: F, t13375: F, t13379: F, t13384: F, t13388: F, t13391: F, t13674: F, t13677: F, t13680: F, t13682: F, t13685: F, t13688: F, t13690: F, t3051: F, t462: F, t92: F, t9903: F, t9907: F, t9935: F, t9958: F, t9960: F) -> (F,) {
    let t13694 = t13693 * t2355;
    let t13697 = 4.0 / 3.0 * t462 * t13353 - 2.0 / 3.0 * t462 * t13357 - 2.0 / 3.0 * t462 * t13362 - 2.0 / 9.0 * t9903 - 8.0 / 27.0 * t9907 + t9958 / 9.0 + 2.0 / 27.0 * t9960 - 6.0 * t462 * t13370 + 4.0 * t462 * t13375 - t9935 + t462 * t13379 / 3.0 + 2.0 / 9.0 * t462 * t13384 - t13388 + 2.0 / 3.0 * t462 * t13391 - t92 * t13674 + 2.0 / 3.0 * t3051 * t13677 - 4.0 / 9.0 * t13680 + 4.0 / 9.0 * t13682 * t13685 - 4.0 / 3.0 * t13688 * t13690 - 4.0 / 3.0 * t13688 * t13694;
    (t13697,)
}
