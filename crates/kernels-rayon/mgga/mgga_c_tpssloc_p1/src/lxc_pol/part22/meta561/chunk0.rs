//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2065/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2065(t275: f64, t2790: f64, t2840: f64, t41654: f64, t41961: f64, t2843: f64, t2928: f64, t315: f64, t2931: f64, t10523: f64, t938: f64, t10660: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42028 = t275 / t2840 / t2790;
    let t42086 = 0.31003950617283950618e1_f64 * t41654;
    let t42087 = 0.13388493827160493828e1_f64 * t41961;
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    let t42102 = 1.0_f64 / t42101;
    let t42109 = t2928 * t2928;
    let t42110 = 1.0_f64 / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = 1.0_f64 / t42112;
    let t42128 = t938 * t10523;
    let t42143 = t888 * t10660;
    (t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42113, t42128, t42143)
}
