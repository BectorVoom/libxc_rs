//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1325/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1325<F: Float>(t23542: F, t23547: F, t23552: F, t23556: F, t23557: F, t27685: F, t27687: F, t27690: F, t27692: F, t27695: F, t27697: F, t27749: F, t27751: F, t27753: F, t27755: F, t31563: F, t32403: F) -> (F,) {
    let t32420 = -t32403 - t31563 - 0.20508037716432813315e4 * t27685 - 0.69263436422725855034e2 * t27687 - 0.20508037716432813316e4 * t23542 + 0.96319466275353142155e0 * t27690 - 0.23392894490538584828e1 * t27692 + 2.0 * t27695 - 48.0 * t27697 - 24.0 * t27749 + t23547 - t23552 + t23556 - 8.0 * t27751 + 160.0 * t27753 - 240.0 * t27755 - 0.5848223622634646207e0 * t23557;
    (t32420,)
}
