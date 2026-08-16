//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 900/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk900(t2528: f64, t448: f64, t1444: f64, t995: f64, t1450: f64, t459: f64, t1466: f64, t1424: f64, t34: f64, t1435: f64, t38: f64, t1437: f64, t1441: f64, t1453: f64, t2490: f64, t2494: f64, t454: f64, t6655: f64, t6659: f64, t6662: f64, t6665: f64, t6668: f64, t6676: f64, t6680: f64, t6683: f64, t6686: f64, t6689: f64, t974: f64, t991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6700 = t2528 * t448;
    let t6703 = t995 * t1444;
    let t6706 = t995 * t1450;
    let t6709 = t2528 * t459;
    let t6712 = t995 * t1466;
    let t6723 = t34 * t1424;
    let t6738 = t38 * t1435;
    let t6747 = 200.0_f64 / 27.0_f64 * t1453 * t974 - 100.0_f64 / 27.0_f64 * t454 * t2490 - 50.0_f64 / 9.0_f64 * t454 * t2494 - 10.0_f64 / 27.0_f64 * t34 * t6655 + 20.0_f64 / 9.0_f64 * t6723 * t6659 + 10.0_f64 / 9.0_f64 * t34 * t6662 + 5.0_f64 / 3.0_f64 * t34 * t6665 - 5.0_f64 * t34 * t6668 - 50.0_f64 / 27.0_f64 * t991 * t1437 - 25.0_f64 / 9.0_f64 * t991 * t1441 - 10.0_f64 / 27.0_f64 * t38 * t6676 - 20.0_f64 / 9.0_f64 * t6738 * t6680 + 10.0_f64 / 9.0_f64 * t38 * t6683 - 5.0_f64 / 3.0_f64 * t38 * t6686 + 5.0_f64 * t38 * t6689;
    (t6700, t6703, t6706, t6709, t6712, t6723, t6738, t6747)
}
