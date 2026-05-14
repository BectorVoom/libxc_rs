//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1168/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1168<F: Float>(t1409: F, t167: F, t1419: F, t532: F, t5801: F, t11920: F, t11951: F, t12085: F, t12087: F, t12089: F, t12091: F, t1404: F, t16349: F, t17019: F, t17021: F, t17024: F, t17027: F, t17028: F, t17037: F, t17041: F, t17045: F, t17047: F, t17048: F, t17051: F, t17054: F, t4023: F, t4059: F, t510: F, t518: F, t538: F) -> (F,) {
    let t17057 = t1409 * t167;
    let t17058 = t17057 * t1419;
    let t17062 = 0.93706135855523581992e-2 * t532 * t5801;
    let t17063 = -t16349 * t538 - 0.28111840756657074598e-1 * t17019 * t17021 - 0.23426533963880895498e-1 * t17024 + t17027 - 0.46853067927761790996e-2 * t510 * t17028 + 0.46853067927761790996e-2 * t4059 * t518 - 0.93706135855523581992e-2 * t12085 - 0.18741227171104716398e-1 * t12087 + 0.23426533963880895498e-2 * t12089 + 0.46853067927761790996e-2 * t12091 - 0.14055920378328537299e-1 * t11920 * t17037 - 0.14055920378328537299e-1 * t1404 * t17041 - t17045 - t17047 + 0.46853067927761790996e-2 * t4023 * t17048 - 0.18741227171104716398e-1 * t11951 * t17051 + 0.46853067927761790996e-2 * t1404 * t17054 - 0.18741227171104716398e-1 * t4059 * t17058 - t17062;
    (t17063,)
}
