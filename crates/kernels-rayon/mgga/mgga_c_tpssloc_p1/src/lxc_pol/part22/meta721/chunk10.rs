//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2354/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2354(t1509: f64, t5631: f64, t5611: f64, t9975: f64, t13380: f64, t13397: f64, t1510: f64, t1523: f64, t16673: f64, t16811: f64, t17030: f64, t20876: f64, t20986: f64, t25115: f64, t2617: f64, t4166: f64, t4281: f64, t4282: f64, t4286: f64, t4291: f64, t58181: f64, t58262: f64, t59331: f64, t67739: f64, t828: f64, t829: f64) -> (f64, f64) {
    let t68217 = t5631 * t1509;
    let t68246 = t9975 * t5611;
    let t68256 = -18.0_f64 * t13397 * t4282 * t68246 * t828 + 6.0_f64 * t13380 * t20986 * t4281 - 3.0_f64 * t1510 * t4291 * t58262 - 3.0_f64 * t1510 * t4291 * t59331 - 3.0_f64 * t17030 * t25115 * t4291 + 6.0_f64 * t4281 * t4282 * t67739 - 3.0_f64 * t4291 * t68217 * t829 - 3.0_f64 * t1523 * t58181 - 3.0_f64 * t16673 * t4286 + 6.0_f64 * t16811 * t4166 - 3.0_f64 * t20876 * t2617;
    (t68217, t68256)
}
