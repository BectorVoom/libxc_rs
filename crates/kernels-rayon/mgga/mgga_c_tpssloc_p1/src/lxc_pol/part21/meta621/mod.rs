//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2398;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta621(t3809: f64, t40281: f64, t12267: f64, t3865: f64, t12344: f64, t3777: f64, t1369: f64, t12250: f64, t3850: f64, t10021: f64, t154: f64, t59: f64, t3749: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t12227: f64, t9577: f64, t116: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40282, t40284, t40292, t40293, t40335, t40341) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2398(t3809, t40281, t12267, t3865, t12344, t3777, t1369, t12250, t3850, t10021, t154, t59);
        let (t40343, t40344, t40347, t40350, t40351, t40353) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2399(t3749, t40341, t59, t598, t535, t795, t215, t39933, t12227, t9577, t116, t557);
    (t40282, t40284, t40292, t40293, t40335, t40341, t40343, t40344, t40347, t40350, t40351, t40353)
}
