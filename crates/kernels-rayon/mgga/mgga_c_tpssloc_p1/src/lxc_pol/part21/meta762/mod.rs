//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2636;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta762(t16235: f64, t54566: f64, t1811: f64, t40005: f64, t12283: f64, t16265: f64, t16257: f64, t16398: f64, t1358: f64, t16347: f64, t40281: f64, t5259: f64, t1336: f64, t1361: f64, t242: f64, t12189: f64, t5206: f64, t40406: f64, t5202: f64, t16115: f64, t3726: f64, t12199: f64, t16111: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54567, t54582, t54585, t54607, t54609, t54611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2636(t16235, t54566, t1811, t40005, t12283, t16265, t16257, t16398, t1358, t16347, t40281, t5259);
        let (t54614, t54631, t54633, t54635, t54637, t54639) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2637(t1336, t1361, t242, t12189, t5206, t40406, t5202, t16115, t3726, t12199, t16111, t1804, t40005);
    (t54567, t54582, t54585, t54607, t54609, t54611, t54614, t54631, t54633, t54635, t54637, t54639)
}
