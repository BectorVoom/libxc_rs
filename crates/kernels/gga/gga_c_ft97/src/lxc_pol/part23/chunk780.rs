//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 780/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk780<F: Float>(t192: F, t19240: F, t852: F, t10586: F, t10594: F, t13682: F, t13688: F, t19669: F, t19672: F, t19675: F, t19678: F, t19681: F, t19684: F, t19687: F, t19691: F, t19693: F, t19695: F, t19699: F, t19703: F, t19706: F, t19711: F, t19716: F, t19720: F, t19723: F, t3139: F, t462: F, t92: F) -> (F,) {
    let t19727 = t192 * t852 * t19240;
    let t19729 = -8.0 / 3.0 * t3139 * t19669 + t462 * t19672 / 3.0 + 2.0 / 3.0 * t462 * t19675 - 2.0 / 9.0 * t462 * t19678 - 4.0 / 3.0 * t3139 * t19681 + 2.0 / 9.0 * t462 * t19684 + 4.0 / 3.0 * t462 * t19687 - 4.0 / 27.0 * t10586 - 2.0 / 9.0 * t19691 + t19693 / 9.0 + 2.0 / 27.0 * t19695 + 2.0 * t462 * t19699 + 4.0 * t462 * t19703 - t462 * t19706 / 3.0 - 6.0 * t462 * t19711 - 4.0 / 3.0 * t13688 * t19716 - 4.0 / 3.0 * t13688 * t19720 + 4.0 / 9.0 * t13682 * t19723 - t92 * t19727 - t10594;
    (t19729,)
}
