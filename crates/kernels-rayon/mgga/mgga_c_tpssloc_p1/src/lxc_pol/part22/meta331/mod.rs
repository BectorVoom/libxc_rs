//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1518;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1519;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta331(t5151: f64, t67: f64, t758: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t1365: f64, t5187: f64, t12365: f64, t1827: f64, t12300: f64, t12418: f64, t820: f64, t1351: f64, t12289: f64, t242: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16169, t16171, t16186, t16191, t16195, t16211, t16214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1518(t5151, t67, t758, t1345, t68, t1799, t1995, t1365, t5187, t12365, t1827, t12300);
        let t16224 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1519(t12418, t820);
        let (t16225, t16232, t16233) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1520(t1351, t1799, t12289, t242, t1336);
    (t16169, t16171, t16186, t16191, t16195, t16211, t16214, t16224, t16225, t16232, t16233)
}
