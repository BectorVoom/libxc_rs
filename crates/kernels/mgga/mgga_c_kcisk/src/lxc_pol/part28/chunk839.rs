//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 839/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk839<F: Float>(t11458: F, t682: F, t1810: F, t1846: F, t1825: F, t5082: F, t5100: F, t680: F, t143: F, t4597: F, t3845: F, t429: F, t686: F, t5814: F, t79: F, t435: F, t690: F) -> (F, F, F, F, F, F, F) {
    let t11460 = 0.14055920378328537299e-1 * t11458 * t682;
    let t11461 = t1846 * t1810;
    let t11463 = t5082 * t1825;
    let t11480 = 1.0 / t5100 / t680;
    let t11495 = t143 * t4597;
    let t11524 = 0.27323333333333333333e-1 * t429 * t3845 * t686;
    let t11525 = t5814 * t79;
    let t11528 = 0.77488888888888888888e-2 * t435 * t11525 * t690;
    (t11460, t11461, t11463, t11480, t11495, t11524, t11528)
}
