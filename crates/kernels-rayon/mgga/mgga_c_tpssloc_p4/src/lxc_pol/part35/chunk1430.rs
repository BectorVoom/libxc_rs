//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1430/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1430(t5456: f64, t7982: f64, t105213: f64, t106733: f64, t106736: f64, t106738: f64, t106741: f64, t106744: f64, t1442: f64, t19451: f64, t20296: f64, t20717: f64, t20720: f64, t2114: f64, t2165: f64, t22425: f64, t27863: f64, t29848: f64, t33690: f64, t510: f64, t5450: f64, t5457: f64, t5493: f64, t5494: f64, t6287: f64, t6468: f64, t652: f64, t7266: f64, t7983: f64, t7989: f64, t8103: f64, t8107: f64) -> (f64, f64) {
    let t108902 = t7982 * t5456;
    let t108918 = -6.0_f64 * t5493 * t652 * t8103 - 6.0_f64 * t108902 * t510 - 3.0_f64 * t1442 * t29848 - 6.0_f64 * t19451 * t7989 - 6.0_f64 * t20296 * t2165 - 6.0_f64 * t20717 * t7266 - 2.0_f64 * t20720 * t7266 - t2114 * t22425 - 6.0_f64 * t27863 * t5494 - 6.0_f64 * t33690 * t5494 - 3.0_f64 * t5450 * t8103 - 6.0_f64 * t5457 * t8103 - 3.0_f64 * t6287 * t7983 + 3.0_f64 * t6468 * t8107 + t105213 - t106733 - t106736 - t106738 + t106741 - t106744;
    (t108902, t108918)
}
