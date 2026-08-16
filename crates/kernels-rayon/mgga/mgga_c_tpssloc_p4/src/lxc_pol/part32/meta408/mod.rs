//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1576;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta408(t4781: f64, t4785: f64, t3313: f64, t11277: f64, t5988: f64, t1117: f64, t11275: f64, t3411: f64, t6106: f64, t1157: f64, t6105: f64, t1164: f64, t11282: f64, t6068: f64, t11285: f64, t1155: f64, t11292: f64, t4883: f64, t15218: f64, t4882: f64, t1190: f64, t6238: f64, t1743: f64, t4965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18264, t18268, t18270, t18273) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1576(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
        let (t18278, t18282, t18285, t18287, t18297) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1577(t11282, t6068, t11285, t1155, t1164, t11292, t4883, t15218, t4882, t1190, t6238, t1743, t4965);
    (t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18287, t18297)
}
