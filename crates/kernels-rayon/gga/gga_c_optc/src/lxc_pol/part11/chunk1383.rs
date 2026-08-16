//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1383/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1383(t1102: f64, t4208: f64, t53361: f64, t1006: f64, t17612: f64, t2325: f64, t5471: f64, t423: f64, t6116: f64, t1582: f64, t1583: f64, t1584: f64, t1588: f64, t17618: f64, t17622: f64, t17635: f64, t18199: f64, t20680: f64, t34107: f64, t4536: f64, t5236: f64, t5238: f64, t5242: f64, t5246: f64, t5256: f64, t53453: f64, t53465: f64) -> (f64, f64, f64) {
    let t58651 = 0.69263023597503453196e2_f64 * t1102 * t53361 * t4208;
    let t58652 = t1006 * t17612;
    let t58656 = t2325 * t5471;
    let t58661 = 1.0_f64 / t423 / t6116;
    let t58668 = 2.0_f64 / 3.0_f64 * t53453 - 100.0_f64 * t17618 * t5256 * t1588 - 1520000.0_f64 / 81.0_f64 * t17622 * t18199 * t5246 + 51260000.0_f64 / 729.0_f64 * t5236 * t5238 / t20680 * t5246 - 160.0_f64 / 81.0_f64 * t34107 - 200.0_f64 / 3.0_f64 * t53465 - t58651 + 200.0_f64 / 27.0_f64 * t58652 * t1584 * t1588 + 40000.0_f64 / 27.0_f64 * t58656 * t5242 * t5246 - 304700.0_f64 / 243.0_f64 * t1582 * t1583 * t58661 * t1588 + 2.0_f64 / 3.0_f64 * t4536 * t17635;
    (t58651, t58661, t58668)
}
