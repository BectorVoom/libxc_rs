//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1328/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1328(t1409: f64, t167: f64, t1419: f64, t532: f64, t5801: f64, t11920: f64, t11951: f64, t12085: f64, t12087: f64, t12089: f64, t12091: f64, t1404: f64, t16349: f64, t17019: f64, t17021: f64, t17024: f64, t17027: f64, t17028: f64, t17037: f64, t17041: f64, t17045: f64, t17047: f64, t17048: f64, t17051: f64, t17054: f64, t4023: f64, t4059: f64, t510: f64, t518: f64, t538: f64) -> f64 {
    let t17057 = t1409 * t167;
    let t17058 = t17057 * t1419;
    let t17062 = 0.93706135855523581992e-2_f64 * t532 * t5801;
    let t17063 = -t16349 * t538 - 0.28111840756657074598e-1_f64 * t17019 * t17021 - 0.23426533963880895498e-1_f64 * t17024 + t17027 - 0.46853067927761790996e-2_f64 * t510 * t17028 + 0.46853067927761790996e-2_f64 * t4059 * t518 - 0.93706135855523581992e-2_f64 * t12085 - 0.18741227171104716398e-1_f64 * t12087 + 0.23426533963880895498e-2_f64 * t12089 + 0.46853067927761790996e-2_f64 * t12091 - 0.14055920378328537299e-1_f64 * t11920 * t17037 - 0.14055920378328537299e-1_f64 * t1404 * t17041 - t17045 - t17047 + 0.46853067927761790996e-2_f64 * t4023 * t17048 - 0.18741227171104716398e-1_f64 * t11951 * t17051 + 0.46853067927761790996e-2_f64 * t1404 * t17054 - 0.18741227171104716398e-1_f64 * t4059 * t17058 - t17062;
    t17063
}
