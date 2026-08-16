//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 876/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk876(t1396: f64, t7257: f64, t1468: f64, t1464: f64, t2011: f64, t5756: f64, t1395: f64, t1364: f64, t4115: f64, t5686: f64, t5764: f64, t5766: f64, t7043: f64, t7092: f64, t7102: f64, t7106: f64, t7109: f64, t7196: f64, t7199: f64, t7205: f64, t7208: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7258 = t1396 * t7257;
    let t7259 = t1468 * t7258;
    let t7260 = t1464 * t7259;
    let t7262 = t5756 * t2011;
    let t7263 = t1395 * t7262;
    let t7264 = t1464 * t7263;
    let t7266 = 0.33163888888888888888e-2_f64 * t5686 - 0.66725e-1_f64 * t1364 * t7092 + 0.22109259259259259258e-2_f64 * t5764 - 0.33163888888888888888e-2_f64 * t5766 + 0.66725e-1_f64 * t1364 * t7043 - t4115 - 0.33163888888888888888e-2_f64 * t7102 + 0.24320185185185185185e-1_f64 * t7106 - 0.13265555555555555555e-1_f64 * t7109 + 0.24872916666666666666e-2_f64 * t7196 + 0.33163888888888888888e-2_f64 * t7199 + 0.16581944444444444444e-2_f64 * t7205 - 0.49745833333333333332e-2_f64 * t7208 - 0.24872916666666666666e-2_f64 * t7260 - 0.88437037037037037034e-2_f64 * t7264;
    (t7258, t7259, t7260, t7262, t7263, t7264, t7266)
}
