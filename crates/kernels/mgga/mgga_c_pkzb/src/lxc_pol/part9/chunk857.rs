//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 857/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk857<F: Float>(t2528: F, t448: F, t1444: F, t995: F, t1450: F, t459: F, t1466: F, t1424: F, t34: F, t1435: F, t38: F, t1437: F, t1441: F, t1453: F, t2490: F, t2494: F, t454: F, t6655: F, t6659: F, t6662: F, t6665: F, t6668: F, t6676: F, t6680: F, t6683: F, t6686: F, t6689: F, t974: F, t991: F) -> (F, F, F, F, F, F, F, F) {
    let t6700 = t2528 * t448;
    let t6703 = t995 * t1444;
    let t6706 = t995 * t1450;
    let t6709 = t2528 * t459;
    let t6712 = t995 * t1466;
    let t6723 = t34 * t1424;
    let t6738 = t38 * t1435;
    let t6747 = 200.0 / 27.0 * t1453 * t974 - 100.0 / 27.0 * t454 * t2490 - 50.0 / 9.0 * t454 * t2494 - 10.0 / 27.0 * t34 * t6655 + 20.0 / 9.0 * t6723 * t6659 + 10.0 / 9.0 * t34 * t6662 + 5.0 / 3.0 * t34 * t6665 - 5.0 * t34 * t6668 - 50.0 / 27.0 * t991 * t1437 - 25.0 / 9.0 * t991 * t1441 - 10.0 / 27.0 * t38 * t6676 - 20.0 / 9.0 * t6738 * t6680 + 10.0 / 9.0 * t38 * t6683 - 5.0 / 3.0 * t38 * t6686 + 5.0 * t38 * t6689;
    (t6700, t6703, t6706, t6709, t6712, t6723, t6738, t6747)
}
