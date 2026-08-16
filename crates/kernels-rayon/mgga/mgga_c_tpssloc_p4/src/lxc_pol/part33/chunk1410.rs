//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1410/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1410(t22574: f64, t28830: f64, t33136: f64, t106956: f64, t1874: f64, t107496: f64, t107499: f64, t107507: f64, t107509: f64, t107512: f64, t107515: f64, t107519: f64, t107521: f64, t107523: f64, t107525: f64, t107527: f64, t107530: f64, t1442: f64, t1458: f64, t1774: f64, t27996: f64, t28811: f64, t33085: f64, t5494: f64, t6287: f64, t6468: f64, t652: f64, t7451: f64, t7681: f64) -> f64 {
    let t107533 = 18.0_f64 * t22574 * t33136 * t28830;
    let t107539 = 6.0_f64 * t106956 * t1874;
    let t107543 = -6.0_f64 * t1458 * t28811 * t652 - 3.0_f64 * t1442 * t28811 - 6.0_f64 * t1774 * t27996 - 6.0_f64 * t33085 * t5494 - 3.0_f64 * t6287 * t7451 + 3.0_f64 * t6468 * t7681 - t107496 - t107499 - t107507 - t107509 + t107512 + t107515 - t107519 - t107521 - t107523 - t107525 - t107527 - t107530 - t107533 - t107539;
    t107543
}
