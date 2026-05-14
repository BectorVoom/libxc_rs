//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 215/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk215<F: Float>(t104: F, t109: F, t101: F, t123: F, t320: F, t105: F, t121: F, t646: F, t650: F, t653: F, t96: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t654 = t104 * t104;
    let t655 = 1.0 / t654;
    let t659 = t109 * tau0;
    let t660 = t101 * t123;
    let t663 = t320 * tau0;
    let t664 = t663 * t101;
    let t667 = -0.10666666666666666667e-1 * t96 * t646 * t105 + 0.42666666666666666668e-4 * t650 * t653 * t655 + 5.0 / 3.0 * t659 * t660 + 5.0 / 3.0 * t121 * t664;
    (t654, t655, t659, t660, t663, t664, t667)
}
