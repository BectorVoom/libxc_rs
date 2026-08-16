//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1793;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta563(t81615: f64, t7524: f64, t81612: f64, t81613: f64, t4250: f64, t81749: f64, t23145: f64, t4166: f64, t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64, t131: f64, t6598: f64, t9537: f64, t225: f64, t2627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87167, t87177, t87197, t87199, t87202, t87205) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1793(t81615, t7524, t81612, t81613, t4250, t81749, t23145, t4166, t22690, t234, t7496, t776, t81792);
        let (t87211, t87213, t87229, t87230) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1794(t23109, t23110, t232, t236, t4233, t25132, t81876, t131, t6598, t9537, t225, t2627);
    (t87167, t87177, t87197, t87199, t87202, t87205, t87211, t87213, t87229, t87230)
}
