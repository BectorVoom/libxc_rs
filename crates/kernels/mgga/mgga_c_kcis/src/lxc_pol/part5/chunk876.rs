//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 876/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk876<F: Float>(t1396: F, t7257: F, t1468: F, t1464: F, t2011: F, t5756: F, t1395: F, t1364: F, t4115: F, t5686: F, t5764: F, t5766: F, t7043: F, t7092: F, t7102: F, t7106: F, t7109: F, t7196: F, t7199: F, t7205: F, t7208: F) -> (F, F, F, F, F, F, F) {
    let t7258 = t1396 * t7257;
    let t7259 = t1468 * t7258;
    let t7260 = t1464 * t7259;
    let t7262 = t5756 * t2011;
    let t7263 = t1395 * t7262;
    let t7264 = t1464 * t7263;
    let t7266 = F::cast_from(0.33163888888888888888e-2_f64) * t5686 - F::new(0.66725e-1) * t1364 * t7092 + F::cast_from(0.22109259259259259258e-2_f64) * t5764 - F::cast_from(0.33163888888888888888e-2_f64) * t5766 + F::new(0.66725e-1) * t1364 * t7043 - t4115 - F::cast_from(0.33163888888888888888e-2_f64) * t7102 + F::cast_from(0.24320185185185185185e-1_f64) * t7106 - F::cast_from(0.13265555555555555555e-1_f64) * t7109 + F::cast_from(0.24872916666666666666e-2_f64) * t7196 + F::cast_from(0.33163888888888888888e-2_f64) * t7199 + F::cast_from(0.16581944444444444444e-2_f64) * t7205 - F::cast_from(0.49745833333333333332e-2_f64) * t7208 - F::cast_from(0.24872916666666666666e-2_f64) * t7260 - F::cast_from(0.88437037037037037034e-2_f64) * t7264;
    (t7258, t7259, t7260, t7262, t7263, t7264, t7266)
}
