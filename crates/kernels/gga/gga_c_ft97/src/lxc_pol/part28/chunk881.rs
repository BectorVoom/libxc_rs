//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 881/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk881<F: Float>(t1526: F, t6512: F, t7705: F, t342: F, t34607: F, t630: F, t1774: F, t6520: F, t7151: F, t32029: F, t34592: F, t1286: F, t1546: F, t34595: F, t11280: F, t137404: F, t137412: F, t137415: F, t137418: F, t137442: F, t1527: F, t22883: F, t25582: F, t25846: F, t25942: F, t25960: F, t26027: F, t28: F, t32026: F, t32038: F, t3266: F, t343: F, t34596: F, t34601: F, t356: F, t379: F, t461: F, t5495: F, t6455: F, t7150: F, t7152: F, t72: F) -> (F,) {
    let t144505 = t1526 * t7705 * t6512;
    let t144511 = t342 * t630 * t34607;
    let t144520 = t7151 * t1774 * t6520;
    let t144524 = t34592 * t32029;
    let t144538 = t1286 * t1546 * t34595;
    let t144551 = -t144505 / 36.0 - t34592 * t32038 / 6.0 - t137404 / 9.0 - t144511 / 12.0 - t25582 * t7150 * t7152 / 6.0 - t7151 * t461 * t26027 / 6.0 + t144520 / 18.0 + t5495 * t34596 / 18.0 + t144524 / 18.0 - t32026 * t34601 / 6.0 - t1526 * t1527 * t25942 / 12.0 - t1526 * t11280 * t25960 / 6.0 - t1286 * t28 * t22883 * t3266 - t144538 / 54.0 + t1286 * t356 * t6455 * t379 / 18.0 + t137412 / 18.0 + t137415 / 18.0 - t137418 / 36.0 - t137442 - t342 * t343 * t72 * t25846 / 4.0;
    (t144551,)
}
