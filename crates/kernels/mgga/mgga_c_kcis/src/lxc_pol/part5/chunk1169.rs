//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1169/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1169<F: Float>(t1419: F, t21620: F, t1650: F, t1961: F, t833: F, t11920: F, t11951: F, t1404: F, t17024: F, t17027: F, t17088: F, t1924: F, t1979: F, t21524: F, t21528: F, t21531: F, t21534: F, t21586: F, t21594: F, t21597: F, t21600: F, t21604: F, t21607: F, t21611: F, t21614: F, t21617: F, t4023: F, t4059: F, t510: F, t5623: F, t5867: F) -> (F, F, F) {
    let t21621 = t21620 * t1419;
    let t21624 = t1650 * t1961;
    let t21625 = t21624 * t833;
    let t21628 = -0.14055920378328537299e-1 * t11920 * t21524 - 0.93706135855523581992e-2 * t4023 * t21528 - 0.18741227171104716398e-1 * t11951 * t21531 - 0.23426533963880895498e-2 * t1404 * t21534 - 0.46853067927761790996e-2 * t510 * t21586 - 2.0 * t5623 * t1979 - 2.0 * t1924 * t5867 - 0.93706135855523581992e-2 * t17024 + t17027 + 0.93706135855523581992e-2 * t1404 * t21594 - 0.18741227171104716398e-1 * t4059 * t21597 + 0.46853067927761790996e-2 * t4023 * t21600 + 0.46853067927761790996e-2 * t1404 * t21604 - 0.14055920378328537299e-1 * t1404 * t21607 - 0.56223681513314149196e-1 * t510 * t21611 + 0.28111840756657074598e-1 * t510 * t21614 + 0.46853067927761790996e-2 * t1404 * t21617 + 0.14055920378328537299e-1 * t510 * t21621 - 0.18741227171104716398e-1 * t17088 * t21625;
    (t21624, t21625, t21628)
}
