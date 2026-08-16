//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3199/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199(t18940: f64, t486: f64, t15753: f64, t4889: f64, t18375: f64, t3536: f64, t11668: f64, t11728: f64, t11734: f64, t1216: f64, t15507: f64, t15594: f64, t15620: f64, t15637: f64, t18300: f64, t19062: f64, t3243: f64, t3506: f64, t3515: f64, t3577: f64, t4582: f64, t4978: f64, t4989: f64, t53378: f64, t53387: f64, t53389: f64, t53397: f64, t53404: f64, t53410: f64, t6219: f64) -> f64 {
    let t66533 = t486 * t18940;
    let t66545 = t4889 * t15753;
    let t66554 = t3536 * t18375;
    let t66564 = -t11734 * t19062 / 1536.0_f64 - t3515 * t4582 * t66533 * t1216 / 1536.0_f64 - t53378 / 1152.0_f64 + 5.0_f64 / 3456.0_f64 * t15594 * t4989 + t3506 * t4582 * t66533 * t4978 / 768.0_f64 - t66545 / 243.0_f64 + t15507 * t15637 / 144.0_f64 - t11728 * t4582 * t18300 * t15620 / 512.0_f64 - t53387 / 108.0_f64 + t66554 / 2304.0_f64 - t53389 / 432.0_f64 + t53397 / 2304.0_f64 + t53404 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t3577 * t11668 * t6219 * t3243 + 5.0_f64 / 5184.0_f64 * t53410;
    t66564
}
