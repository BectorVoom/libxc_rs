//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2162;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta514(t17635: f64, t4588: f64, t4582: f64, t1023: f64, t5681: f64, t3071: f64, t248: f64, t3101: f64, t5878: f64, t3039: f64, t3051: f64, t5685: f64, t1041: f64, t4630: f64, t4641: f64, t5873: f64, t3130: f64, t376: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17642, t17643, t17648, t17649, t17655, t17656, t17659) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2162(t17635, t4588, t4582, t1023, t5681, t3071, t248, t3101, t5878, t3039, t3051, t5685);
        let (t17660, t17662, t17667, t17668, t17670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2163(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5872);
    (t17642, t17643, t17648, t17649, t17655, t17656, t17659, t17660, t17662, t17667, t17668, t17670)
}
