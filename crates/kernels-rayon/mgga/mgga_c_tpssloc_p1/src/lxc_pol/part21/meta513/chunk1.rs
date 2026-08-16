//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2161/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2161(t17635: f64, t4583: f64, t4582: f64, t1041: f64, t13948: f64, t13952: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t17616: f64, t17621: f64, t17625: f64, t17632: f64, t2960: f64, t3039: f64, t5885: f64, t5890: f64, t5894: f64) -> (f64, f64, f64) {
    let t17636 = t4583 * t17635;
    let t17637 = t4582 * t17636;
    let t17640 = t17616 / 864.0_f64 - t2960 * t5894 / 81.0_f64 + t17621 / 648.0_f64 + t13948 + t13952 + t13959 + t13963 - t13966 / 6912.0_f64 - t17625 / 432.0_f64 - t2960 * t5890 / 108.0_f64 + t2960 * t5885 / 54.0_f64 - t3039 * t17632 / 1536.0_f64 - t1041 * t17637 / 2304.0_f64 - t13972;
    (t17636, t17637, t17640)
}
