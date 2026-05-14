//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 856/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk856<F: Float>(t1436: F, t4810: F, t983: F, t1435: F, t23: F, t1430: F, t444: F, t1440: F, t2499: F, t1429: F, t27: F, t2503: F, t82: F, t1419: F, t1437: F, t1441: F, t2490: F, t2494: F, t434: F, t6655: F, t6658: F, t6659: F, t6662: F, t6665: F, t6668: F, t7: F, t974: F, t980: F) -> (F, F, F, F, F, F, F) {
    let t6676 = t4810 * t983 * t1436;
    let t6679 = t23 * t1435;
    let t6680 = t1430 * t444;
    let t6683 = t2499 * t1440;
    let t6686 = t27 * t1429;
    let t6689 = t2503 * t82;
    let t6692 = 440.0 / 27.0 * t1419 * t974 - 160.0 / 27.0 * t434 * t2490 - 80.0 / 9.0 * t434 * t2494 - 10.0 / 27.0 * t7 * t6655 + 20.0 / 9.0 * t6658 * t6659 + 10.0 / 9.0 * t7 * t6662 + 5.0 / 3.0 * t7 * t6665 - 5.0 * t7 * t6668 - 80.0 / 27.0 * t980 * t1437 - 40.0 / 9.0 * t980 * t1441 - 10.0 / 27.0 * t23 * t6676 - 20.0 / 9.0 * t6679 * t6680 + 10.0 / 9.0 * t23 * t6683 - 5.0 / 3.0 * t23 * t6686 + 5.0 * t23 * t6689;
    (t6676, t6679, t6680, t6683, t6686, t6689, t6692)
}
