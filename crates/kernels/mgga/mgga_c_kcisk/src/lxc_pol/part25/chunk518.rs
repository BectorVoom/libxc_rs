//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 518/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk518<F: Float>(t586: F, t4705: F, t4742: F, t4636: F, t4638: F, t4642: F, t4646: F, t4650: F, t600: F, t1670: F, t45: F, t1675: F, t596: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4743 = t586 * t586;
    let t4744 = 1.0 / t4743;
    let t4745 = t4705 * t4744;
    let t4747 = 0.16081824322151104822e2 * t4742 * t4745;
    let t4748 = 0.12361111111111111111e-1 * t4636;
    let t4753 = t4748 + 0.61805555555555555556e-2 * t4638 - 0.61805555555555555555e-2 * t4642 + 0.18541666666666666667e-1 * t4646 - 0.92708333333333333333e-2 * t4650;
    let t4754 = t4753 * t600;
    let t4757 = t45 * t1670;
    let t4760 = t1675 * t596;
    let t4761 = 1.0 / t4760;
    (t4743, t4744, t4745, t4747, t4748, t4753, t4754, t4757, t4761)
}
