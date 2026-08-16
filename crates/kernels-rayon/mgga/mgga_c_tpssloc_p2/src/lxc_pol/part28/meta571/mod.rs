//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1851;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta571(t4250: f64, t81749: f64, t23145: f64, t4166: f64, t2649: f64, t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64, t13336: f64, t1898: f64, t249: f64, t23047: f64, t2635: f64, t1516: f64, t81766: f64, t23127: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87197, t87200, t87202, t87205, t87211) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1851(t4250, t81749, t23145, t4166, t2649, t22690, t234, t7496, t776, t81792, t23109, t23110, t232, t236, t4233);
        let (t87213, t87216, t87219, t87222, t87224) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1852(t25132, t81876, t13336, t1898, t249, t23047, t4166, t2635, t1516, t81766, t23127, t4261);
    (t87197, t87200, t87202, t87205, t87211, t87213, t87216, t87219, t87222, t87224)
}
