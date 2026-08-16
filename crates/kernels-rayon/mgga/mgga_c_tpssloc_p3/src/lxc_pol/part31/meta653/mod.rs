//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta653(t16932: f64, t25084: f64, t16937: f64, t16907: f64, t23146: f64, t17009: f64, t17013: f64, t25111: f64, t7496: f64, t87447: f64, t22690: f64, t23122: f64, t5544: f64, t841: f64, t23097: f64, t5617: f64, t776: f64, t815: f64, t1510: f64, t4233: f64, t6605: f64, t232: f64, t58688: f64, t5612: f64, t1509: f64, t4119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98631, t98633, t98635, t98637, t98639, t98642, t98647) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1932(t16932, t25084, t16937, t16907, t23146, t17009, t17013, t25111, t7496, t87447, t22690, t23122, t5544, t841);
        let (t98651, t98655, t98659, t98663, t98668) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1933(t23097, t5617, t776, t815, t1510, t4233, t6605, t232, t58688, t5612, t1509, t4119);
    (t98631, t98633, t98635, t98637, t98639, t98642, t98647, t98651, t98655, t98659, t98663, t98668)
}
