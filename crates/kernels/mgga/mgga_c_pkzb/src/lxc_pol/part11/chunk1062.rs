//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1062/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1062<F: Float>(t10415: F, t10418: F, t10423: F, t10428: F, t19396: F, t2500: F, t28649: F, t28653: F, t28659: F, t28662: F, t28665: F, t28671: F, t28677: F, t3324: F, t434: F, t445: F, t6658: F, t7: F) -> (F,) {
    let t28792 = 80.0 / 81.0 * t434 * t10415 + 40.0 / 81.0 * t7 * t28649 - 10.0 / 9.0 * t19396 * t28653 - 80.0 / 9.0 * t434 * t10418 - 10.0 / 9.0 * t19396 * t28659 + 10.0 / 3.0 * t6658 * t28662 + 10.0 / 3.0 * t7 * t28665 - 40.0 / 9.0 * t434 * t10423 + 10.0 / 9.0 * t7 * t28671 + 5.0 / 3.0 * t7 * t28677 - 6160.0 / 81.0 * t10428 * t445 + 880.0 / 27.0 * t3324 * t2500;
    (t28792,)
}
