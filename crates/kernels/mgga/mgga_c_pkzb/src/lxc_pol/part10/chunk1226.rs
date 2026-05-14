//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1226/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1226<F: Float>(t1440: F, t8645: F, t1435: F, t1541: F, t78: F, t444: F, t8657: F, t8653: F, t980: F, t82: F, t8624: F, t8649: F, t1419: F, t1437: F, t1441: F, t19393: F, t19444: F, t19455: F, t23: F, t3315: F, t3319: F, t3324: F, t434: F, t6658: F, t6679: F, t6680: F, t6683: F, t6686: F, t8625: F, t8636: F) -> (F, F, F, F, F, F, F) {
    let t23782 = t8645 * t1440;
    let t23786 = t1435 * t78 * t1541;
    let t23790 = t1435 * t8657 * t444;
    let t23793 = t8653 * t1440;
    let t23796 = t980 * t1435;
    let t23811 = t8624 * t82;
    let t23814 = t8649 * t82;
    let t23819 = -160.0 / 27.0 * t980 * t6683 - 10.0 / 27.0 * t23 * t23782 + 20.0 / 9.0 * t23 * t23786 + 20.0 / 9.0 * t23 * t23790 + 10.0 / 9.0 * t23 * t23793 + 320.0 / 27.0 * t23796 * t6680 + t19444 - t19455 + 880.0 / 81.0 * t1419 * t3315 + 440.0 / 27.0 * t1419 * t3319 + 880.0 / 81.0 * t3324 * t1437 + 80.0 / 9.0 * t980 * t6686 - 80.0 / 9.0 * t434 * t8636 + 440.0 / 27.0 * t3324 * t1441 - 20.0 / 3.0 * t6658 * t23811 + 20.0 / 3.0 * t6679 * t23814 - 320.0 / 27.0 * t19393 * t8625;
    (t23782, t23786, t23790, t23793, t23811, t23814, t23819)
}
