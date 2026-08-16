//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3186/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186(t11668: f64, t11678: f64, t11692: f64, t15478: f64, t15569: f64, t15659: f64, t18395: f64, t18946: f64, t19000: f64, t3577: f64, t3578: f64, t45114: f64, t45128: f64, t4723: f64, t52893: f64, t52897: f64, t52908: f64, t52917: f64, t52926: f64, t52932: f64, t53176: f64, t65014: f64, t65452: f64, t66073: f64, t66076: f64, t66079: f64, t66084: f64, t66092: f64) -> f64 {
    let t66111 = t66073 / 3456.0_f64 - t66076 / 1728.0_f64 - t66079 / 1728.0_f64 + t52908 / 1152.0_f64 - t52917 / 864.0_f64 - t66084 / 576.0_f64 + t15569 * t15478 / 216.0_f64 - t45114 * t52897 * t15659 * t53176 / 128.0_f64 + t66092 / 576.0_f64 - 5.0_f64 / 1296.0_f64 * t52893 * t45128 * t65014 + t52926 / 324.0_f64 + t52932 / 54.0_f64 + 5.0_f64 / 3456.0_f64 * t3577 * t11668 * t4723 * t65452 - t11678 * t3578 * t18946 * t19000 / 576.0_f64 + t11692 * t3578 * t53176 * t18395 / 1152.0_f64;
    t66111
}
