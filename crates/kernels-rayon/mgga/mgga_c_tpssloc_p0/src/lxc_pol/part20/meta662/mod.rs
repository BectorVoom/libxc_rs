//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta662(t1102: f64, t3279: f64, t14801: f64, t14804: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50827: f64, t50828: f64, t50832: f64, t50834: f64, t1667: f64, t9709: f64, t14712: f64, t699: f64, t1113: f64, t136: f64, t50830: f64, t2403: f64, t4778: f64, t4723: f64, t9258: f64, t3297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50837, t50839, t50845) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482(t1102, t3279, t14801, t14804, t43727, t43729, t43748, t43750, t50824, t50827, t50828, t50832, t50834);
        let (t50846, t50848, t50851, t50853, t50854, t50857, t50859) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483(t1667, t9709, t14712, t699, t1113, t136, t50830, t2403, t4778, t4723, t9258, t3297);
    (t50837, t50839, t50845, t50846, t50848, t50851, t50853, t50854, t50857, t50859)
}
