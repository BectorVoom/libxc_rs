//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 948/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk948(t2022: f64, t7961: f64, t118573: f64, t118586: f64, t118588: f64, t118596: f64, t118602: f64, t120350: f64, t120363: f64, t120375: f64, t120393: f64, t120416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122864 = t2022 * t7961;
    let t123566 = 0.32298204875312312682e-2_f64 * t118573;
    let t123571 = 0.5383034145885385447e-3_f64 * t118586;
    let t123572 = 7.0_f64 / 144.0_f64 * t118588;
    let t123576 = 7.0_f64 / 576.0_f64 * t118596;
    let t123578 = 7.0_f64 / 576.0_f64 * t118602;
    let t124139 = 7.0_f64 / 576.0_f64 * t120350;
    let t124142 = 0.5383034145885385447e-3_f64 * t120363;
    let t124146 = 7.0_f64 / 144.0_f64 * t120375;
    let t124154 = 0.32298204875312312682e-2_f64 * t120393;
    let t124163 = 7.0_f64 / 576.0_f64 * t120416;
    (t122864, t123566, t123571, t123572, t123576, t123578, t124139, t124142, t124146, t124154, t124163)
}
