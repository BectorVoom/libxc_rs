//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta699(t5154: f64, t9905: f64, t15968: f64, t67: f64, t758: f64, t17: f64, t750: f64, t2225: f64, t5166: f64, t15921: f64, t592: f64, t39478: f64, t15977: f64, t2516: f64, t5151: f64, t1787: f64, t9861: f64, t15971: f64, t39491: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54393, t54396, t54399, t54401, t54403, t54404) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666(t5154, t9905, t15968, t67, t758, t17, t750, t2225, t5166, t15921, t592, t39478);
        let (t54406, t54409, t54411, t54413, t54414, t54415) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667(t15977, t592, t17, t2516, t5151, t1787, t9861, t15971, t39491, t39463, t39468, t39472, t39476, t39483, t39490, t54393, t54396, t54399, t54401, t54403, t54404);
    (t54393, t54396, t54399, t54401, t54403, t54404, t54406, t54409, t54411, t54413, t54414, t54415)
}
