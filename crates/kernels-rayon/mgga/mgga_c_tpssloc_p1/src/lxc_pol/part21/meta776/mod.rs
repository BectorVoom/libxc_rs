//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2685;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta776(t2221: f64, t6328: f64, t2223: f64, t2225: f64, t39571: f64, t17: f64, t2516: f64, t6320: f64, t19572: f64, t750: f64, t184: f64, t56349: f64, t1388: f64, t5356: f64, t15899: f64, t39570: f64, t39585: f64, t39590: f64, t39593: f64, t5160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56391, t56393, t56395, t56396, t56398, t56401, t56403) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2685(t2221, t6328, t2223, t2225, t39571, t17, t2516, t6320, t19572, t750, t184, t56349);
        let t56408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686(t1388, t5356, t15899, t39570, t39585, t39590, t39593, t5160, t56391, t56393, t56395, t56396, t56398, t56401, t56403);
    (t56391, t56393, t56395, t56396, t56398, t56401, t56403, t56408)
}
