//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 958/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk958(t12571: f64, t31863: f64, t45844: f64, t8662: f64, t33676: f64, t9239: f64, t118573: f64, t118586: f64, t118588: f64, t118596: f64, t118602: f64, t120350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122976 = t12571 * t31863;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123566 = 0.32298204875312312682e-2_f64 * t118573;
    let t123571 = 0.5383034145885385447e-3_f64 * t118586;
    let t123572 = 7.0_f64 / 144.0_f64 * t118588;
    let t123576 = 7.0_f64 / 576.0_f64 * t118596;
    let t123578 = 7.0_f64 / 576.0_f64 * t118602;
    let t124139 = 7.0_f64 / 576.0_f64 * t120350;
    (t122976, t122988, t123001, t123566, t123571, t123572, t123576, t123578, t124139)
}
