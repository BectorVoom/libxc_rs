//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2223;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta540(t11282: f64, t6068: f64, t11285: f64, t1155: f64, t1164: f64, t11292: f64, t4883: f64, t15218: f64, t4882: f64, t1190: f64, t6238: f64, t1743: f64, t4965: f64, t486: f64, t6224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18274, t18276, t18278, t18279, t18280, t18282, t18283, t18285, t18287, t18297) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2223(t11282, t6068, t11285, t1155, t1164, t11292, t4883, t15218, t4882, t1190, t6238, t1743, t4965);
        let t18300 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2224(t486, t6224);
    (t18274, t18276, t18278, t18279, t18280, t18282, t18283, t18285, t18287, t18297, t18300)
}
