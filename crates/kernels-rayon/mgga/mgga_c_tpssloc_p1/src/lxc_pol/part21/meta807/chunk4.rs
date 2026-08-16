//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2814/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814(t17087: f64, t225: f64, t17060: f64, t13050: f64, t13071: f64, t13072: f64, t13377: f64, t13460: f64, t13463: f64, t1492: f64, t1527: f64, t1528: f64, t17022: f64, t17049: f64, t17050: f64, t17057: f64, t25168: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t4147: f64, t4268: f64, t4273: f64, t46452: f64, t46488: f64, t47585: f64, t5637: f64, t798: f64, t855: f64, t865: f64, t866: f64, t9593: f64) -> f64 {
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    let t59518 = 2.0_f64 * t1492 * t13377 * t259 + 4.0_f64 * t855 * t2718 * t1527 * t13460 + 8.0_f64 * t13463 * t4273 - 12.0_f64 * t4147 * t13050 - 2.0_f64 * t46452 * t1528 - 2.0_f64 * t47585 * t1528 + 4.0_f64 * t855 * t2718 * t17049 * t865 - 2.0_f64 * t2713 * t17050 - 4.0_f64 * t59498 * t866 + 4.0_f64 * t2597 * t17057 - 2.0_f64 * t59503 * t866 - 24.0_f64 * t25168 * t46488 * t13071 + 4.0_f64 * t9593 * t5637 + 8.0_f64 * t4268 * t13072 - 12.0_f64 * t4268 * t13050 + 2.0_f64 * t798 * t17022 * t259;
    t59518
}
