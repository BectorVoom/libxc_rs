//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 814/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk814(t3340: f64, t448: f64, t459: f64, t2528: f64, t995: f64, t3356: f64, t2500: f64, t2504: f64, t3315: f64, t3319: f64, t3347: f64, t34: f64, t38: f64, t445: f64, t454: f64, t6723: f64, t6738: f64, t8621: f64, t8625: f64, t8631: f64, t8636: f64, t8646: f64, t8650: f64, t8654: f64, t8658: f64, t991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8664 = t3340 * t448;
    let t8667 = t3340 * t459;
    let t8670 = t995 * t2528;
    let t8673 = t3356 * t448;
    let t8676 = t3356 * t459;
    let t8705 = -50.0_f64 / 27.0_f64 * t454 * t3315 - 10.0_f64 / 27.0_f64 * t34 * t8621 + 20.0_f64 / 9.0_f64 * t6723 * t8625 - 25.0_f64 / 9.0_f64 * t454 * t3319 + 10.0_f64 / 9.0_f64 * t34 * t8631 + 5.0_f64 / 3.0_f64 * t34 * t8636 + 200.0_f64 / 27.0_f64 * t3347 * t445 - 100.0_f64 / 27.0_f64 * t991 * t2500 + 50.0_f64 / 9.0_f64 * t991 * t2504 - 10.0_f64 / 27.0_f64 * t38 * t8646 - 20.0_f64 / 9.0_f64 * t6738 * t8650 + 10.0_f64 / 9.0_f64 * t38 * t8654 + 5.0_f64 / 3.0_f64 * t38 * t8658;
    (t8664, t8667, t8670, t8673, t8676, t8705)
}
