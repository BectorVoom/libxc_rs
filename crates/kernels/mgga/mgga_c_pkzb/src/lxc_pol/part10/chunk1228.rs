//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1228/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1228<F: Float>(t1435: F, t991: F, t1437: F, t1441: F, t1453: F, t19520: F, t19545: F, t19551: F, t23736: F, t23747: F, t23750: F, t23754: F, t23790: F, t23811: F, t23814: F, t3315: F, t3319: F, t3347: F, t34: F, t38: F, t454: F, t6680: F, t6686: F, t6723: F, t6738: F, t8625: F, t8636: F) -> (F,) {
    let t23870 = t991 * t1435;
    let t23887 = 20.0 / 9.0 * t38 * t23790 + 20.0 / 9.0 * t34 * t23736 + 40.0 / 81.0 * t34 * t23747 + 20.0 / 9.0 * t34 * t23750 - 10.0 / 27.0 * t34 * t23754 - 20.0 / 3.0 * t6723 * t23811 + 20.0 / 3.0 * t6738 * t23814 + 200.0 / 27.0 * t23870 * t6680 - 200.0 / 27.0 * t19520 * t8625 + t19545 - t19551 - 50.0 / 9.0 * t454 * t8636 + 200.0 / 27.0 * t3347 * t1441 + 400.0 / 81.0 * t1453 * t3315 + 200.0 / 27.0 * t1453 * t3319 + 400.0 / 81.0 * t3347 * t1437 + 50.0 / 9.0 * t991 * t6686;
    (t23887,)
}
