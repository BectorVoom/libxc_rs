//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1327/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1327<F: Float>(t221: F, t2627: F, t4509: F, t23587: F, t23592: F, t23596: F, t23597: F, t23637: F, t23645: F, t23652: F, t23656: F, t23660: F, t23661: F, t23668: F, t27786: F, t27789: F, t27792: F, t27794: F, t27796: F, t27798: F) -> (F,) {
    let t32441 = t4509 * t221 * t2627;
    let t32447 = 4.0 * t27786 + 2.0 * t27789 + t23587 - t23592 - t23596 - 0.70178683471615754484e1 * t23597 - t23637 - t23645 - 0.11393789434848516923e-2 * t27792 + 0.10843581300301739842e-1 * t32441 + t23652 - 24.0 * t27794 - 48.0 * t27796 - t23656 - 64.0 * t27798 + t23660 + 0.20779030926817756511e3 * t23661 + t23668;
    (t32447,)
}
