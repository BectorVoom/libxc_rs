//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1703;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta329(t10022: f64, t248: f64, t557: f64, t555: f64, t10027: f64, t541: f64, t12267: f64, t1362: f64, t3777: f64, t3865: f64, t1369: f64, t1361: f64, t2690: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12328, t12330, t12335, t12336, t12339) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1703(t10022, t248, t557, t555, t10027, t541, t12267, t1362, t3777, t3865);
        let (t12340, t12344, t12345) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1704(t12339, t1369, t1361, t2690, t1336);
    (t12328, t12330, t12335, t12336, t12339, t12340, t12344, t12345)
}
