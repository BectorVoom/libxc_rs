//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 231/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk231(t245: f64, t395: f64, t163: f64, t394: f64, t158: f64, t401: f64, t402: f64, t954: f64, t957: f64, t960: f64, t964: f64, t966: f64, t969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1055 = t245 * t395;
    let t1059 = t394 * t163;
    let t1060 = 1.0_f64 / t1059;
    let t1061 = t158 * t1060;
    let t1062 = t401 * t401;
    let t1063 = t1062 * t402;
    let t1072 = -0.78438333333333333333e0_f64 * t954 + 0.15687666666666666667e1_f64 * t957 + 0.68863333333333333333e0_f64 * t960 + 0.14025833333333333333e0_f64 * t964 + 0.28051666666666666667e0_f64 * t966 + 0.17365833333333333333e0_f64 * t969;
    (t1055, t1060, t1061, t1062, t1063, t1072)
}
