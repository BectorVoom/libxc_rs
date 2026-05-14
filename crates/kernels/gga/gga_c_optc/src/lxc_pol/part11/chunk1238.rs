//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1238/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1238<F: Float>(t1102: F, t4208: F, t53361: F, t1006: F, t17612: F, t2325: F, t5471: F, t423: F, t6116: F, t1582: F, t1583: F, t1584: F, t1588: F, t17618: F, t17622: F, t17635: F, t18199: F, t20680: F, t34107: F, t4536: F, t5236: F, t5238: F, t5242: F, t5246: F, t5256: F, t53453: F, t53465: F) -> (F, F, F) {
    let t58651 = 0.69263023597503453196e2 * t1102 * t53361 * t4208;
    let t58652 = t1006 * t17612;
    let t58656 = t2325 * t5471;
    let t58661 = 1.0 / t423 / t6116;
    let t58668 = 2.0 / 3.0 * t53453 - 100.0 * t17618 * t5256 * t1588 - 1520000.0 / 81.0 * t17622 * t18199 * t5246 + 51260000.0 / 729.0 * t5236 * t5238 / t20680 * t5246 - 160.0 / 81.0 * t34107 - 200.0 / 3.0 * t53465 - t58651 + 200.0 / 27.0 * t58652 * t1584 * t1588 + 40000.0 / 27.0 * t58656 * t5242 * t5246 - 304700.0 / 243.0 * t1582 * t1583 * t58661 * t1588 + 2.0 / 3.0 * t4536 * t17635;
    (t58651, t58661, t58668)
}
