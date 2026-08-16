//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 931/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk931(t1437: f64, t21020: f64, t1451: f64, t7186: f64, t738: f64, t104: f64, t111: f64, t11967: f64, t120: f64, t12003: f64, t12005: f64, t12009: f64, t1404: f64, t1445: f64, t17137: f64, t17143: f64, t18431: f64, t21685: f64, t21688: f64, t21691: f64, t21694: f64, t21704: f64, t21706: f64, t21708: f64, t4858: f64) -> f64 {
    let t21711 = t1437 * t21020;
    let t21714 = t1451 * t21020;
    let t21717 = t738 * t7186;
    let t21719 = -0.21078e-1_f64 * t104 * t21685 - 0.28104e-1_f64 * t4858 * t21688 - 0.1585e-2_f64 * t111 * t21691 - 0.52833333333333333333e-3_f64 * t111 * t21694 + t11967 - 0.10929333333333333333e-1_f64 * t12003 + 0.35222222222222222222e-2_f64 * t12005 + 0.39210208333333333333e-4_f64 * t12009 + 0.11955719325063177623e-1_f64 * t1404 * t18431 - 0.5179538907796306876e-4_f64 * t1445 * t18431 + t17137 - t17143 - 0.15613333333333333333e-2_f64 * t21704 + 0.23526125e-4_f64 * t21706 - 0.3513e-2_f64 * t104 * t21708 + 0.7925e-3_f64 * t111 * t21711 + 0.50413125e-5_f64 * t120 * t21714 + 0.26416666666666666667e-2_f64 * t21717;
    t21719
}
