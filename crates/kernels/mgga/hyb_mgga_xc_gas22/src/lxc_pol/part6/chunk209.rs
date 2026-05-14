//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 209/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk209<F: Float>(t313: F, t95: F, t317: F, t97: F, t98: F, t104: F, t109: F, t101: F, t123: F, t310: F, t105: F, t121: F, t647: F, t96: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t651 = t95 * t313;
    let t652 = t317 * t97;
    let t654 = 1.0 / t98 / t652;
    let t655 = t104 * t104;
    let t656 = 1.0 / t655;
    let t660 = t109 * tau0;
    let t661 = t101 * t123;
    let t664 = t310 * tau0;
    let t665 = t664 * t101;
    let t668 = -0.10241644597362152e-1 * t96 * t647 * t105 + 0.39334231522004008709e-4 * t651 * t654 * t656 + 5.0 / 3.0 * t660 * t661 + 5.0 / 3.0 * t121 * t665;
    (t651, t652, t654, t655, t656, t660, t661, t664, t665, t668)
}
