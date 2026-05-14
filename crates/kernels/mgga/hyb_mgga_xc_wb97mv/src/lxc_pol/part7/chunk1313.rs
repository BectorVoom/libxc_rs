//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1313/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1313<F: Float>(t27474: F, t27545: F, t27572: F, t27575: F, t31631: F, t31633: F, t31636: F, t31640: F, t31642: F, t31644: F, t31646: F, t31648: F, t31652: F, t31654: F, t31656: F, t31658: F, t31660: F, t31663: F, t31666: F, t9554: F, t9565: F, t975: F) -> (F,) {
    let t32074 = 0.8276162067083744048e4 * t27545 * t27474 * t975 - 0.4155806185363551302e3 * t27572 * t9565 + 0.14035736694323150897e2 * t27575 * t9554 - t31631 - t31633 - t31636 - t31640 + t31642 + t31644 + t31646 + t31648 - t31652 - t31654 - t31656 - t31658 - t31660 - t31663 - t31666;
    (t32074,)
}
