//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1706;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta331(t12365: f64, t1354: f64, t1307: f64, t3792: f64, t3788: f64, t835: f64, t1336: f64, t3795: f64, t3799: f64, t3853: f64, t3858: f64, t12267: f64, t1340: f64, t3719: f64, t550: f64, t1995: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12366, t12369, t12384, t12385, t12386, t12388, t12395, t12397) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1706(t12365, t1354, t1307, t3792, t3788, t835, t1336, t3795, t3799, t3853, t3858, t12267, t1340);
        let (t12407, t12418, t12419) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1707(t3719, t550, t1995, t67, t246);
    (t12366, t12369, t12384, t12385, t12386, t12388, t12395, t12397, t12407, t12418, t12419)
}
