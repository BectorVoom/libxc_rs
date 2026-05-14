//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1378/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1378<F: Float>(t22958: F, t33017: F, t6713: F, t116145: F, t1799: F, t6986: F, t112269: F, t116513: F, t116971: F, t116979: F, t121730: F, t121733: F, t121736: F, t121739: F, t121748: F, t121751: F, t121754: F, t34018: F, t34073: F, t34218: F, t35123: F) -> (F, F, F) {
    let t121757 = t6713 * t33017 * t22958;
    let t121760 = t1799 * t116145 * t6986;
    let t121762 = 0.66327777777777777776e-2 * t121730 - 0.22109259259259259259e-2 * t121733 + 0.66327777777777777776e-2 * t121736 - 0.55273148148148148147e-2 * t121739 - 0.92592592592592592594e-2 * t116513 * t34018 + 0.26805555555555555556e-2 * t112269 * t35123 - t116971 + t116979 + 0.20833333333333333334e-1 * t34073 * t34218 - 0.33163888888888888888e-2 * t121748 - 0.33163888888888888888e-2 * t121751 - 0.33163888888888888888e-2 * t121754 + 0.66327777777777777776e-2 * t121757 + 0.88437037037037037034e-2 * t121760;
    (t121757, t121760, t121762)
}
