//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2625;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta752(t40281: f64, t5303: f64, t12211: f64, t16300: f64, t5247: f64, t820: f64, t12250: f64, t1824: f64, t16288: f64, t3853: f64, t12384: f64, t5234: f64, t3795: f64, t5293: f64, t12283: f64, t16405: f64, t40167: f64, t1799: f64, t3791: f64, t40138: f64, t5259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53997, t54003, t54013, t54014, t54034, t54042) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2625(t40281, t5303, t12211, t16300, t5247, t820, t12250, t1824, t16288, t3853, t12384, t5234);
        let (t54043, t54047, t54059, t54063, t54068, t54086) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2626(t3795, t54042, t40281, t5293, t12283, t16405, t40167, t820, t1799, t3791, t40138, t5259);
    (t53997, t54003, t54013, t54014, t54034, t54043, t54047, t54059, t54063, t54068, t54086)
}
