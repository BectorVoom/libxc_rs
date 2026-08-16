//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2046;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta469(t1343: f64, t16206: f64, t820: f64, t12365: f64, t1827: f64, t12300: f64, t1799: f64, t3734: f64, t12351: f64, t12418: f64, t1351: f64, t3807: f64, t12289: f64, t242: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16208, t16211, t16214, t16215, t16217, t16224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2046(t1343, t16206, t820, t12365, t1827, t12300, t1799, t3734, t12351, t12418);
        let (t16225, t16226, t16227, t16232, t16233) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2047(t1351, t1799, t3807, t16224, t12289, t242, t1336);
    (t16208, t16211, t16214, t16215, t16217, t16224, t16225, t16226, t16227, t16232, t16233)
}
