//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta662<F: Float>(t1102: F, t3279: F, t14801: F, t14804: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50827: F, t50828: F, t50832: F, t50834: F, t1667: F, t9709: F, t14712: F, t699: F, t1113: F, t136: F, t50830: F, t2403: F, t4778: F, t4723: F, t9258: F, t3297: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50837, t50839, t50845) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482::<F>(t1102, t3279, t14801, t14804, t43727, t43729, t43748, t43750, t50824, t50827, t50828, t50832, t50834);
        let (t50846, t50848, t50851, t50853, t50854, t50857, t50859) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483::<F>(t1667, t9709, t14712, t699, t1113, t136, t50830, t2403, t4778, t4723, t9258, t3297);
    (t50837, t50839, t50845, t50846, t50848, t50851, t50853, t50854, t50857, t50859)
}
