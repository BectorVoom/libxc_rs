//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 963/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk963(t1071: f64, t160: f64, t829: f64, t239: f64, t330: f64, t822: f64, t1057: f64, t2466: f64, t1065: f64, t2471: f64, t323: f64, t325: f64, t8291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10108 = t160 * t1071;
    let t10109 = t10108 * t829;
    let t10112 = t160 * t239;
    let t10113 = 0.71734315950379065738e-1_f64 * t10112;
    let t10114 = t822 * t330;
    let t10115 = 0.62154466893555682512e-3_f64 * t10114;
    let t10131 = t2466 * t1057;
    let t10133 = t2471 * t1065;
    let t10137 = 0.77488888888888888888e-2_f64 * t323 * t8291 * t325;
    (t10108, t10109, t10112, t10113, t10114, t10115, t10131, t10133, t10137)
}
