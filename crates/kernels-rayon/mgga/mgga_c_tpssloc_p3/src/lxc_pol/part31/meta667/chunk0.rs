//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1960/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960(t12725: f64, t1458: f64, t1774: f64, t1849: f64, t1983: f64, t19924: f64, t20085: f64, t2096: f64, t22574: f64, t2314: f64, t24432: f64, t24995: f64, t26114: f64, t26179: f64, t26558: f64, t26870: f64, t26967: f64, t27163: f64, t27188: f64, t27215: f64, t28030: f64, t29201: f64, t29205: f64, t29243: f64, t33234: f64, t4034: f64, t4073: f64, t652: f64, t6876: f64, t7057: f64, t7217: f64, t74060: f64, t7458: f64, t7796: f64, t7802: f64, t9016: f64, t97804: f64, t97911: f64) -> f64 {
    let t101134 = -4.0_f64 * t12725 * t7802 - 4.0_f64 * t27188 * t4073 + t97804 * t2096 - 2.0_f64 * t6876 * t29201 - 6.0_f64 * t22574 * t24432 * t74060 + 12.0_f64 * t24995 * t9016 * t19924 + 2.0_f64 * t27215 * t1849 - 2.0_f64 * t28030 * t7057 - 4.0_f64 * t26114 * t7796 - 4.0_f64 * t26179 * t7796 - 4.0_f64 * t7458 * t27163 - 2.0_f64 * t26967 * t1774 + 2.0_f64 * t1983 * t7217 * t20085 - 4.0_f64 * t2314 * t29205 - 4.0_f64 * t4034 * t29205 - 4.0_f64 * t652 * t26870 * t1458 + 2.0_f64 * t6876 * t29243 + 12.0_f64 * t22574 * t26558 * t97911 - 4.0_f64 * t33234 * t4073;
    t101134
}
