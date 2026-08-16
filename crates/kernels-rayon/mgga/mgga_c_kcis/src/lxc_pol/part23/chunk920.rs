//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 920/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk920(t167: f64, t17102: f64, t11952: f64, t11954: f64, t11958: f64, t11960: f64, t11962: f64, t11967: f64, t11974: f64, t11977: f64, t11985: f64, t11987: f64, t11995: f64, t12003: f64, t12005: f64, t12009: f64, t17096: f64, t17098: f64, t17100: f64) -> f64 {
    let t17103 = t17102 * t167;
    let t17118 = 0.23911438650126355246e-1_f64 * t17096 - 0.31077233446777841256e-3_f64 * t17098 + 0.11955719325063177623e0_f64 * t17100 - 0.72513544709148296264e-3_f64 * t17103 - 0.62154466893555682512e-3_f64 * t11952 + 0.10359077815592613752e-3_f64 * t11954 + 0.23911438650126355246e-1_f64 * t11958 + 0.47822877300252710492e-1_f64 * t11960 - 0.11955719325063177623e-1_f64 * t11962 + t11967 - 0.117630625e-4_f64 * t11974 + 0.15684083333333333333e-4_f64 * t11977 + 0.4684e-2_f64 * t11985 - 0.15613333333333333333e-2_f64 * t11987 - 0.9368e-2_f64 * t11995 - 0.21858666666666666666e-1_f64 * t12003 + 0.70444444444444444443e-2_f64 * t12005 + 0.78420416666666666666e-4_f64 * t12009;
    t17118
}
