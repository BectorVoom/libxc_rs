//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 183/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk183<F: Float>(t522: F, t536: F, t505: F, t507: F, t511: F, t513: F, t516: F, t523: F, t529: F, t532: F, t535: F, t455: F, t3: F, t5: F) -> (F, F, F, F, F) {
    let t537 = t536 * t522;
    let t540 = param_c_os_0 + t505 * t507 + t511 * t513 + 0.3e-2 * t516 * t523 + t529 * t532 + 0.3e-2 * t535 * t537;
    let t542 = 1.0 / t455;
    let t543 = t3 * t542;
    let t544 = t5 - t543;
    (t537, t540, t542, t543, t544)
}
