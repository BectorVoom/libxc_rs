//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1179/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1179<F: Float>(t116796: F, t116338: F, t446: F, t7824: F, t101876: F, t101879: F, t102258: F, t102261: F, t116776: F, t116781: F, t116783: F, t116786: F, t116790: F, t116794: F, t116296: F, t116336: F, t116379: F, t116410: F, t116442: F, t116469: F, t116506: F, t116539: F, t116566: F, t116590: F, t116633: F, t116672: F, t116705: F, t116731: F, t116773: F, t488: F) -> (F, F) {
    let t116797 = t116796 / 12.0;
    let t116799 = t446 * t7824 * t116338;
    let t116802 = t116776 / 6.0 + 16.0 / 9.0 * t101876 + t116781 - t116783 - 6.0 * t116786 + 2.0 / 3.0 * t116790 + t116794 - t116797 - 2.0 / 3.0 * t116799 + 8.0 / 9.0 * t101879 - t102258 - t102261;
    let t116807 = t488 * (t116296 + t116336 + t116379 + t116410 + t116442 + t116469 + t116506 + t116539 + t116566 + t116590 + t116633 + t116672 + t116705 + t116731 + t116773 + t116802);
    (t116799, t116807)
}
