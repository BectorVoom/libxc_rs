//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta633(t54555: f64, t12289: f64, t1336: f64, t836: f64, t1811: f64, t40005: f64, t40281: f64, t5259: f64, t1361: f64, t242: f64, t12189: f64, t5206: f64, t40406: f64, t5202: f64, t12199: f64, t16111: f64, t1804: f64, t2585: f64, t3732: f64, t46853: f64, t5308: f64, t16118: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54556, t54566, t54582, t54612, t54614, t54631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168(t54555, t12289, t1336, t836, t1811, t40005, t40281, t5259, t1361, t242, t12189, t5206);
        let (t54633, t54638, t54639, t54644, t54663) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2169(t40406, t5202, t12199, t16111, t1804, t40005, t2585, t3732, t46853, t5308, t16118, t9577);
    (t54556, t54566, t54582, t54612, t54614, t54631, t54633, t54638, t54639, t54644, t54663)
}
