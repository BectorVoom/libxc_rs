//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1591;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta388<F: Float>(t1100: F, t14758: F, t1667: F, t2403: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t11219: F, t14726: F, t136: F, t4775: F, t699: F, t14736: F, t3297: F, t14740: F, t14731: F, t1113: F, t14749: F, t14753: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14759, t14766, t14776) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1591::<F>(t1100, t14758, t1667, t2403, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1592::<F>(t11219, t14726, t136, t4775, t699, t14736, t3297, t14740, t14731, t1113, t14749, t14753);
    (t14759, t14766, t14776, t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795)
}
