//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1884;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta441<F: Float>(t3287: F, t4756: F, t1102: F, t3279: F, t4764: F, t4772: F, t699: F, t1107: F, t14758: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t11211: F, t11213: F, t11369: F, t11372: F, t14702: F, t14705: F, t14708: F, t14711: F, t14713: F, t14759: F, t14776: F, t14779: F, t14782: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F) -> (F, F, F, F, F, F) {
        let (t14813, t14814, t14816, t14818, t14824, t14827) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1884::<F>(t3287, t4756, t1102, t3279, t4764, t4772, t699, t1107, t14758, t11137, t11139, t11141, t11143, t14728, t14809, t14811);
        let t14829 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1885::<F>(t11211, t11213, t11369, t11372, t14702, t14705, t14708, t14711, t14713, t14759, t14776, t14779, t14782, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t14827);
    (t14813, t14814, t14816, t14818, t14824, t14829)
}
