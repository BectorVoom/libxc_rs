//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1080/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1080(t1092: f64, t18497: f64, t2861: f64, t6615: f64, t2855: f64, t6613: f64, t1096: f64, t10463: f64, t13102: f64, t13103: f64, t18461: f64, t18465: f64, t18468: f64, t18471: f64, t18474: f64, t18477: f64, t18483: f64, t18486: f64, t18495: f64, t2836: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t18498 = t1092 * t18497;
    let t18500 = t2861 * t6615;
    let t18502 = t2855 * t6613;
    let t18503 = t1096 * t18502;
    let t18504 = t1092 * t18503;
    let t18506 = -0.24872916666666666666e-2_f64 * t18461 + 0.16581944444444444444e-2_f64 * t18465 + 0.49745833333333333332e-2_f64 * t18468 - t13102 + 0.22109259259259259259e-2_f64 * t13103 + 0.22109259259259259259e-2_f64 * t18471 - 0.33163888888888888888e-2_f64 * t18474 + 0.890445125e-2_f64 * t2836 * t18477 + 0.66725e-1_f64 * t979 * t18477 - 0.178244852896875e-2_f64 * t10463 * t18483 + 0.178089025e-1_f64 * t2836 * t18486 - 0.13345e0_f64 * t979 * t18483 - 0.2671335375e-1_f64 * t2836 * t18483 + 0.13345e0_f64 * t979 * t18486 + 0.22109259259259259259e-2_f64 * t18495 - 0.88437037037037037035e-2_f64 * t18498 - 0.16581944444444444444e-2_f64 * t18500 + 0.66327777777777777776e-2_f64 * t18504;
    (t18498, t18500, t18504, t18506)
}
