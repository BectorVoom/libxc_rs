//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1004/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1004<F: Float>(t11280: F, t1286: F, t137404: F, t137412: F, t137415: F, t137418: F, t137442: F, t144505: F, t144511: F, t144520: F, t144524: F, t144538: F, t1526: F, t1527: F, t22883: F, t25582: F, t25846: F, t25942: F, t25960: F, t26027: F, t28: F, t32026: F, t32038: F, t3266: F, t342: F, t343: F, t34592: F, t34596: F, t34601: F, t356: F, t379: F, t461: F, t5495: F, t6455: F, t7150: F, t7151: F, t7152: F, t72: F) -> F {
    let t144551 = -t144505 / F::new(36.0) - t34592 * t32038 / F::new(6.0) - t137404 / F::new(9.0) - t144511 / F::new(12.0) - t25582 * t7150 * t7152 / F::new(6.0) - t7151 * t461 * t26027 / F::new(6.0) + t144520 / F::new(18.0) + t5495 * t34596 / F::new(18.0) + t144524 / F::new(18.0) - t32026 * t34601 / F::new(6.0) - t1526 * t1527 * t25942 / F::new(12.0) - t1526 * t11280 * t25960 / F::new(6.0) - t1286 * t28 * t22883 * t3266 - t144538 / F::new(54.0) + t1286 * t356 * t6455 * t379 / F::new(18.0) + t137412 / F::new(18.0) + t137415 / F::new(18.0) - t137418 / F::new(36.0) - t137442 - t342 * t343 * t72 * t25846 / F::new(4.0);
    t144551
}
