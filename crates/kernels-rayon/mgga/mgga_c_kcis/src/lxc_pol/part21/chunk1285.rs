//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1285/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1285(t1003: f64, t26685: f64, t27772: f64, t27773: f64, t27812: f64, t3040: f64, t7703: f64, t7706: f64, t95537: f64, t95629: f64, t95636: f64, t95640: f64, t95645: f64, t95649: f64, t95653: f64, t95658: f64, t95662: f64, t95666: f64, t95670: f64) -> f64 {
    let t95681 = -0.13901041666666666667e-2_f64 * t7703 * t95636 - 0.46336805555555555556e-3_f64 * t95640 * t7706 - 0.13901041666666666667e-2_f64 * t7703 * t95645 - 0.69505208333333333333e-3_f64 * t7703 * t95649 - 0.33163888888888888888e-2_f64 * t95653 - 0.1492375e-1_f64 * t95658 - 0.2653111111111111111e-1_f64 * t95662 - 0.33163888888888888888e-2_f64 * t95666 + 0.111403033060546875e-3_f64 * t27812 * t95537 - 0.27802083333333333334e-2_f64 * t7703 * t27772 * t95670 * t1003 - 0.13901041666666666667e-2_f64 * t7703 * t27772 * t27773 * t3040 - 0.2782641015625e-3_f64 * t26685 * t95629;
    t95681
}
