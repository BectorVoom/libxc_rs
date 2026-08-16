//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta486(t4300: f64, t6571: f64, t6553: f64, t1880: f64, t1902: f64, t4142: f64, t1492: f64, t6624: f64, t1519: f64, t214: f64, t6572: f64, t13053: f64, t1528: f64, t1912: f64, t23235: f64, t23281: f64, t25200: f64, t25206: f64, t25209: f64, t25211: f64, t25214: f64, t259: f64, t2713: f64, t7538: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25216, t25217, t25218, t25220, t25222, t25224) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1792(t4300, t6571, t6553, t1880, t1902, t4142, t1492, t6624, t1519, t214);
        let (t25225, t25228) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1793(t25224, t6572, t1880, t13053, t1528, t1912, t23235, t23281, t25200, t25206, t25209, t25211, t25214, t25218, t25220, t25222, t259, t2713, t7538, t855);
    (t25216, t25217, t25220, t25222, t25224, t25225, t25228)
}
