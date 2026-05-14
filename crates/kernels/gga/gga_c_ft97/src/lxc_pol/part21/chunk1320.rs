//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1320/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1320<F: Float>(t1882: F, t30548: F, t30532: F, t106651: F, t106798: F, t119450: F, t120112: F, t12277: F, t1359: F, t144: F, t16942: F, t17365: F, t17369: F, t17510: F, t1901: F, t2185: F, t23470: F, t30244: F, t30383: F, t3590: F, t379: F, t446: F, t47659: F, t4805: F, t574: F, t5842: F, t605: F, t616: F, t6615: F, t6699: F, t9144: F, t95837: F) -> (F,) {
    let t121077 = t1882 * t30548;
    let t121110 = t1882 * t30532;
    let t121122 = -2.0 / 27.0 * t121077 + t446 * t574 * t605 * t5842 * t4805 / 3.0 - t446 * t144 * t120112 / 3.0 - 8.0 / 81.0 * t106798 + 4.0 / 9.0 * t47659 * t95837 * t17365 + 4.0 / 3.0 * t47659 * t106651 * t17369 + 2.0 / 3.0 * t446 * t574 * t12277 * t6699 - t1901 * t9144 * t30383 * t379 / 9.0 - t446 * t574 * t17510 * t1359 / 3.0 - 2.0 / 3.0 * t446 * t574 * t3590 * t6615 - 4.0 / 9.0 * t121110 + 4.0 / 3.0 * t446 * t2185 * t616 * t30244 - 2.0 * t446 * t144 * t119450 - 2.0 / 3.0 * t1901 * t23470 * t16942;
    (t121122,)
}
