//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 763/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk763<F: Float>(t9677: F, t2804: F, t2807: F, t9682: F, t9685: F, t9689: F, t9692: F, t9721: F, t9725: F, t9728: F, t9733: F, t9738: F, t9740: F, t9743: F, t9748: F, t2041: F, t2811: F) -> (F, F, F) {
    let t9753 = 0.11607361111111111111e-2 * t9677;
    let t9758 = -0.52083333333333333333e-2 * t9721 * t2807 + 0.20104166666666666667e-2 * t9725 * t9728 - 0.52083333333333333333e-2 * t9733 * t2807 - t9738 - 0.17361111111111111111e-2 * t9740 * t9743 + 0.52083333333333333333e-2 * t2804 * t9748 + 0.52083333333333333333e-2 * t2804 * t9728 + t9753 + 0.11607361111111111111e-2 * t9682 + 0.17411041666666666666e-2 * t9685 - 0.17411041666666666666e-2 * t9689 + 0.11607361111111111111e-2 * t9692;
    let t9760 = t2811 * t2041;
    (t9753, t9758, t9760)
}
