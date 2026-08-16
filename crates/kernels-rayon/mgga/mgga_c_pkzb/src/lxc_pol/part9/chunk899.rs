//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 899/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk899(t1430: f64, t444: f64, t1440: f64, t2499: f64, t1429: f64, t27: f64, t2503: f64, t82: f64, t1419: f64, t1437: f64, t1441: f64, t23: f64, t2490: f64, t2494: f64, t434: f64, t6655: f64, t6658: f64, t6659: f64, t6662: f64, t6665: f64, t6668: f64, t6676: f64, t6679: f64, t7: f64, t974: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t6680 = t1430 * t444;
    let t6683 = t2499 * t1440;
    let t6686 = t27 * t1429;
    let t6689 = t2503 * t82;
    let t6692 = 440.0_f64 / 27.0_f64 * t1419 * t974 - 160.0_f64 / 27.0_f64 * t434 * t2490 - 80.0_f64 / 9.0_f64 * t434 * t2494 - 10.0_f64 / 27.0_f64 * t7 * t6655 + 20.0_f64 / 9.0_f64 * t6658 * t6659 + 10.0_f64 / 9.0_f64 * t7 * t6662 + 5.0_f64 / 3.0_f64 * t7 * t6665 - 5.0_f64 * t7 * t6668 - 80.0_f64 / 27.0_f64 * t980 * t1437 - 40.0_f64 / 9.0_f64 * t980 * t1441 - 10.0_f64 / 27.0_f64 * t23 * t6676 - 20.0_f64 / 9.0_f64 * t6679 * t6680 + 10.0_f64 / 9.0_f64 * t23 * t6683 - 5.0_f64 / 3.0_f64 * t23 * t6686 + 5.0_f64 * t23 * t6689;
    (t6680, t6683, t6686, t6689, t6692)
}
