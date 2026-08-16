//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta547(t2221: f64, t3826: f64, t12132: f64, t592: f64, t1336: f64, t1339: f64, t2691: f64, t12344: f64, t3777: f64, t10021: f64, t154: f64, t59: f64, t3749: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t12227: f64, t9577: f64, t116: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40225, t40230, t40281, t40292, t40341) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2044(t2221, t3826, t12132, t592, t1336, t1339, t2691, t12344, t3777, t10021, t154, t59);
        let (t40343, t40344, t40347, t40350, t40351, t40353) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2045(t3749, t40341, t59, t598, t535, t795, t215, t39933, t12227, t9577, t116, t557);
    (t40225, t40230, t40281, t40292, t40341, t40343, t40344, t40347, t40350, t40351, t40353)
}
