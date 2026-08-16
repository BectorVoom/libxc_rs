//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1154/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1154(t273: f64, t41654: f64, t242: f64, t281: f64, t283: f64, t275: f64, t2790: f64, t2840: f64, t2843: f64, t2928: f64, t315: f64, t2931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41942 = f64::powf(t273, -0.25e1_f64);
    let t41959 = 0.31310740740740740741e1_f64 * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = 0.13490888888888888889e1_f64 * t41961;
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
    (t41942, t41959, t41961, t41962, t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42112)
}
