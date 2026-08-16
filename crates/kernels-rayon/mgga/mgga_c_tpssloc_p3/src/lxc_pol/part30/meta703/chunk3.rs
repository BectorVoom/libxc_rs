//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2288/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288(t1409: f64, t1597: f64, t23562: f64, t343: f64, t40: f64, t5836: f64, t99645: f64, t18041: f64, t23419: f64, t17649: f64, t17998: f64, t6747: f64, t7583: f64, t83025: f64, t83028: f64, t88348: f64, t88479: f64, t88488: f64) -> (f64, f64, f64) {
    let t99660 = t1409 * t1597;
    let t99662 = t23562 * t99660 * t343;
    let t99665 = t40 * t5836;
    let t99667 = t23562 * t99665 * t343;
    let t99671 = t23562 * t99645 * t343;
    let t99680 = t23419 * t18041;
    let t99682 = 5.0_f64 / 6912.0_f64 * t23419 * t17998 - 0.20186378047070195428e-3_f64 * t99662 * t6747 - 0.10093189023535097714e-3_f64 * t99667 * t6747 - 0.10093189023535097714e-3_f64 * t99671 * t6747 + 0.16149102437656156342e-2_f64 * t88348 * t7583 - t23419 * t17649 / 1152.0_f64 + t83025 / 162.0_f64 + t83028 - t88479 / 3456.0_f64 + t88488 + t99680 / 1728.0_f64;
    (t99660, t99665, t99682)
}
