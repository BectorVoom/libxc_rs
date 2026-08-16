//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1704;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta366(t2363: f64, t88: f64, t1454: f64, t2281: f64, t4044: f64, t626: f64, t4068: f64, t1453: f64, t2332: f64, t9365: f64, t2331: f64, t4067: f64, t666: f64, t2358: f64, t4043: f64, t1444: f64, t2342: f64, t9384: f64, t2341: f64, t92: f64, t2219: f64, t659: f64, t2248: f64, t4049: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12739, t12747, t12750, t12752, t12754, t12757) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1704(t2363, t88, t1454, t2281, t4044, t626, t4068, t1453, t2332, t9365, t2331, t4067);
        let (t12758, t12761, t12771, t12774, t12775, t12778) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1705(t12757, t666, t2358, t4043, t1444, t2342, t9384, t2341, t92, t2219, t659, t2248, t4049);
    (t12739, t12747, t12750, t12752, t12754, t12757, t12758, t12761, t12771, t12774, t12775, t12778)
}
