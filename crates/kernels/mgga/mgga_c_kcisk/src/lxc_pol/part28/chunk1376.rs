//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1376/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1376<F: Float>(t112549: F, t112552: F, t112560: F, t116921: F, t121241: F, t121246: F, t121385: F, t121531: F, t121699: F, t121702: F, t121705: F, t121708: F, t121712: F, t121715: F, t121724: F, t33002: F, t33031: F, t33056: F, t9649: F) -> (F,) {
    let t121726 = -0.69444444444444444447e-2 * t33031 * t121385 + 0.23148148148148148149e-2 * t121699 + 0.11054629629629629629e-2 * t121702 - 0.33163888888888888888e-2 * t121705 - 0.16581944444444444444e-2 * t121708 + 0.23148148148148148149e-2 * t112549 + t112552 - 0.15432098765432098766e-2 * t121712 - 0.66327777777777777776e-2 * t121715 - t112560 + 0.26805555555555555556e-2 * t33056 * t121531 - t116921 - 0.24125e-1 * t9649 * t121241 + 0.13968375e-1 * t33002 * t121246 - 0.49745833333333333332e-2 * t121724;
    (t121726,)
}
