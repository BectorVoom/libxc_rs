//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1017/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1017(t118586: f64, t118588: f64, t118596: f64, t118602: f64, t112829: f64, t114724: f64, t114725: f64, t114736: f64, t116608: f64, t116610: f64, t116613: f64, t116615: f64, t118590: f64, t118592: f64, t118594: f64, t118606: f64, t118608: f64, t118610: f64, t118612: f64) -> f64 {
    let t123571 = 0.5383034145885385447e-3_f64 * t118586;
    let t123572 = 7.0_f64 / 144.0_f64 * t118588;
    let t123576 = 7.0_f64 / 576.0_f64 * t118596;
    let t123578 = 7.0_f64 / 576.0_f64 * t118602;
    let t123583 = t123571 + t123572 - t118590 / 96.0_f64 - t118592 / 96.0_f64 - t118594 / 96.0_f64 + t123576 + t114724 + t114725 + 0.22608743412718618877e-1_f64 * t112829 - t123578 + t116608 - t116610 - 0.19378922925187387609e-1_f64 * t118606 - t118608 / 384.0_f64 + t118610 / 96.0_f64 + t118612 / 96.0_f64 - t114736 + t116613 + t116615;
    t123583
}
