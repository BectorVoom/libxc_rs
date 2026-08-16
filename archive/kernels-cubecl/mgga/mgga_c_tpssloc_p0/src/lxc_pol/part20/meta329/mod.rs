//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1614;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta329<F: Float>(t1174: F, t11754: F, t11692: F, t11694: F, t11699: F, t11703: F, t11705: F, t11709: F, t11719: F, t11724: F, t11728: F, t11731: F, t11734: F, t11738: F, t11741: F, t11746: F, t11748: F, t11751: F, t3511: F, t3518: F, t11153: F, t3439: F, t9288: F, t974: F, t11147: F, t11545: F, t11660: F, t1216: F, t4582: F, t10913: F, t4987: F, t3247: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11755, t11757) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1614::<F>(t1174, t11754, t11692, t11694, t11699, t11703, t11705, t11709, t11719, t11724, t11728, t11731, t11734, t11738, t11741, t11746, t11748, t11751, t3511, t3518);
        let (t11760, t11761, t11765, t11766, t11769, t11770, t11773, t11774, t11778) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1615::<F>(t11153, t3439, t9288, t974, t11147, t11545, t11660, t1216, t4582, t10913, t4987, t3247, t415);
    (t11755, t11757, t11760, t11761, t11765, t11766, t11769, t11770, t11773, t11774, t11778)
}
