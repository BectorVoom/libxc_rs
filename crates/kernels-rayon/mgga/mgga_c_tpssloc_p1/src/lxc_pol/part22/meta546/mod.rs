//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2042;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta546(t1369: f64, t40059: f64, t22843: f64, t241: f64, t67: f64, t10021: f64, t1336: f64, t1339: f64, t1354: f64, t12384: f64, t3777: f64, t12282: f64, t12328: f64, t1333: f64, t2690: f64, t3788: f64, t6924: f64, t246: f64, t12250: f64, t1307: f64, t39037: f64, t522: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40060, t40070, t40123, t40124, t40130, t40138) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2042(t1369, t40059, t22843, t241, t67, t10021, t1336, t1339, t1354, t12384, t3777, t12282);
        let (t40145, t40159, t40167, t40168, t40192, t40224) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2043(t12328, t1333, t1336, t2690, t3788, t67, t6924, t246, t12250, t1307, t39037, t522);
    (t40060, t40070, t40123, t40124, t40130, t40138, t40145, t40159, t40167, t40168, t40192, t40224)
}
