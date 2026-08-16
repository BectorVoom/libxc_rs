//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk605;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk606;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta118(t1694: f64, t3375: f64, t1171: f64, t1706: f64, t1420: f64, t972: f64, t1709: f64, t3431: f64, t1174: f64, t3439: f64, t60: f64, t461: f64, t1409: f64, t3450: f64, t3448: f64, t135: f64, t1716: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4874, t4887, t4889) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk605(t1694, t3375, t1171, t1706, t1420, t972);
        let (t4896, t4897, t4899, t4900, t4904) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk606(t1709, t3431, t1174, t3439, t60, t461, t1409, t3450);
        let (t4908, t4916, t4917, t4919) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk607(t3448, t461, t135, t1716, t1174, t1714);
    (t4874, t4887, t4889, t4896, t4897, t4899, t4900, t4904, t4908, t4916, t4917, t4919)
}
