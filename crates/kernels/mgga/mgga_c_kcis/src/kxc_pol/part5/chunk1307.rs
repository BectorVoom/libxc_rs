//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1307/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1307<F: Float>(t1419: F, t21610: F, t5526: F, t5804: F, t7142: F, t833: F, t4035: F, t7141: F, t1650: F, t1961: F, t11920: F, t11951: F, t1404: F, t17024: F, t17027: F, t17088: F, t1924: F, t1979: F, t21524: F, t21528: F, t21531: F, t21534: F, t21586: F, t21594: F, t21597: F, t21600: F, t21604: F, t21607: F, t4023: F, t4059: F, t510: F, t5623: F, t5867: F) -> (F, F, F) {
    let t21611 = t21610 * t1419;
    let t21614 = t5804 * t5526;
    let t21617 = t7142 * t833;
    let t21620 = t4035 * t7141;
    let t21621 = t21620 * t1419;
    let t21624 = t1650 * t1961;
    let t21625 = t21624 * t833;
    let t21628 = -F::new(0.14055920378328537299e-1) * t11920 * t21524 - F::new(0.93706135855523581992e-2) * t4023 * t21528 - F::new(0.18741227171104716398e-1) * t11951 * t21531 - F::new(0.23426533963880895498e-2) * t1404 * t21534 - F::new(0.46853067927761790996e-2) * t510 * t21586 - F::new(2.0) * t5623 * t1979 - F::new(2.0) * t1924 * t5867 - F::new(0.93706135855523581992e-2) * t17024 + t17027 + F::new(0.93706135855523581992e-2) * t1404 * t21594 - F::new(0.18741227171104716398e-1) * t4059 * t21597 + F::new(0.46853067927761790996e-2) * t4023 * t21600 + F::new(0.46853067927761790996e-2) * t1404 * t21604 - F::new(0.14055920378328537299e-1) * t1404 * t21607 - F::new(0.56223681513314149196e-1) * t510 * t21611 + F::new(0.28111840756657074598e-1) * t510 * t21614 + F::new(0.46853067927761790996e-2) * t1404 * t21617 + F::new(0.14055920378328537299e-1) * t510 * t21621 - F::new(0.18741227171104716398e-1) * t17088 * t21625;
    (t21624, t21625, t21628)
}
