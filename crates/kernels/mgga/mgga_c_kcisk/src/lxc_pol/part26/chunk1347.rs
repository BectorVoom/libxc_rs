//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1347/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1347<F: Float>(t114059: F, t114107: F, t114195: F, t114225: F, t114796: F, t114799: F, t114803: F, t119675: F, t119685: F, t119688: F, t119693: F, t119698: F, t119701: F, t33346: F, t33460: F, t9429: F, t9796: F) -> (F,) {
    let t119703 = 0.22109259259259259259e-2 * t114796 - 0.24872916666666666666e-2 * t119675 + t114799 + 0.8041666666666666667e-2 * t33460 * t33346 + 0.8041666666666666667e-2 * t114195 * t9796 + 0.8041666666666666667e-2 * t114107 * t9796 - 0.46296296296296296297e-2 * t114803 - 0.33163888888888888888e-2 * t119685 + 0.40208333333333333335e-2 * t119688 * t9429 - 0.21444444444444444445e-1 * t114225 * t9796 + 0.26805555555555555557e-2 * t119693 - 0.55555555555555555557e-1 * t114059 * t9796 + 0.66327777777777777776e-2 * t119698 - 0.22109259259259259259e-2 * t119701;
    (t119703,)
}
