//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1307/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1307(t1419: f64, t21610: f64, t5526: f64, t5804: f64, t7142: f64, t833: f64, t4035: f64, t7141: f64, t1650: f64, t1961: f64, t11920: f64, t11951: f64, t1404: f64, t17024: f64, t17027: f64, t17088: f64, t1924: f64, t1979: f64, t21524: f64, t21528: f64, t21531: f64, t21534: f64, t21586: f64, t21594: f64, t21597: f64, t21600: f64, t21604: f64, t21607: f64, t4023: f64, t4059: f64, t510: f64, t5623: f64, t5867: f64) -> (f64, f64, f64) {
    let t21611 = t21610 * t1419;
    let t21614 = t5804 * t5526;
    let t21617 = t7142 * t833;
    let t21620 = t4035 * t7141;
    let t21621 = t21620 * t1419;
    let t21624 = t1650 * t1961;
    let t21625 = t21624 * t833;
    let t21628 = -0.14055920378328537299e-1_f64 * t11920 * t21524 - 0.93706135855523581992e-2_f64 * t4023 * t21528 - 0.18741227171104716398e-1_f64 * t11951 * t21531 - 0.23426533963880895498e-2_f64 * t1404 * t21534 - 0.46853067927761790996e-2_f64 * t510 * t21586 - 2.0_f64 * t5623 * t1979 - 2.0_f64 * t1924 * t5867 - 0.93706135855523581992e-2_f64 * t17024 + t17027 + 0.93706135855523581992e-2_f64 * t1404 * t21594 - 0.18741227171104716398e-1_f64 * t4059 * t21597 + 0.46853067927761790996e-2_f64 * t4023 * t21600 + 0.46853067927761790996e-2_f64 * t1404 * t21604 - 0.14055920378328537299e-1_f64 * t1404 * t21607 - 0.56223681513314149196e-1_f64 * t510 * t21611 + 0.28111840756657074598e-1_f64 * t510 * t21614 + 0.46853067927761790996e-2_f64 * t1404 * t21617 + 0.14055920378328537299e-1_f64 * t510 * t21621 - 0.18741227171104716398e-1_f64 * t17088 * t21625;
    (t21624, t21625, t21628)
}
