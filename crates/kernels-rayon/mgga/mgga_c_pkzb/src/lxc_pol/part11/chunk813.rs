//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 813/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk813(t1429: f64, t8649: f64, t1435: f64, t3333: f64, t444: f64, t8635: f64, t27: f64, t23: f64, t2500: f64, t2504: f64, t3315: f64, t3319: f64, t3324: f64, t434: f64, t445: f64, t6658: f64, t6679: f64, t7: f64, t8621: f64, t8625: f64, t8631: f64, t8636: f64, t8646: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t8650 = t8649 * t1429;
    let t8653 = t1435 * t3333;
    let t8654 = t8653 * t444;
    let t8657 = -t8635;
    let t8658 = t27 * t8657;
    let t8661 = -80.0_f64 / 27.0_f64 * t434 * t3315 - 10.0_f64 / 27.0_f64 * t7 * t8621 + 20.0_f64 / 9.0_f64 * t6658 * t8625 - 40.0_f64 / 9.0_f64 * t434 * t3319 + 10.0_f64 / 9.0_f64 * t7 * t8631 + 5.0_f64 / 3.0_f64 * t7 * t8636 + 440.0_f64 / 27.0_f64 * t3324 * t445 - 160.0_f64 / 27.0_f64 * t980 * t2500 + 80.0_f64 / 9.0_f64 * t980 * t2504 - 10.0_f64 / 27.0_f64 * t23 * t8646 - 20.0_f64 / 9.0_f64 * t6679 * t8650 + 10.0_f64 / 9.0_f64 * t23 * t8654 + 5.0_f64 / 3.0_f64 * t23 * t8658;
    (t8650, t8654, t8657, t8658, t8661)
}
