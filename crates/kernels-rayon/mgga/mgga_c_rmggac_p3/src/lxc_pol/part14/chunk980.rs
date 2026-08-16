//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 980/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk980(t275: f64, t9064: f64, t4928: f64, t645: f64, t903: f64, t1679: f64, t7197: f64, t7200: f64, t38530: f64, t7484: f64, t7450: f64, t10792: f64, t1288: f64, t2376: f64, t2405: f64, t26291: f64, t27102: f64, t36464: f64, t40716: f64, t40719: f64, t40721: f64, t40724: f64, t40725: f64, t40732: f64, t40736: f64, t40740: f64, t40747: f64, t5928: f64, t72: f64, t739: f64, t7574: f64, t7703: f64) -> (f64, f64) {
    let t40750 = 2.0_f64 * t275 * t9064;
    let t40756 = t645 * t4928;
    let t40757 = t903 * t40756;
    let t40759 = t1679 * t7197;
    let t40760 = t40759 * t7200;
    let t40762 = t38530 * t7484;
    let t40764 = t38530 * t7450;
    let t40766 = t40716 - 0.43368970657079495312e-4_f64 * t40719 - 0.71845450211182851384e0_f64 * t26291 * t40721 - 0.71845450211182851384e0_f64 * t40724 * t40725 - 0.35922725105591425692e0_f64 * t739 * t7703 * t27102 + 0.17025839957319135759e-4_f64 * t40732 - 0.20455996240684006296e-1_f64 * t40736 + 0.27274661654245341728e-1_f64 * t40740 - 0.59871208509319042821e-1_f64 * t10792 * t2376 + 0.85129199786595678796e-5_f64 * t40747 + t40750 - 0.24829349937757072982e-4_f64 * t36464 + 0.39914139006212695214e-1_f64 * t5928 * t7574 + t72 * t1288 * t2405 + 0.44903406381989282115e-1_f64 * t40757 - 0.81823984962736025184e-1_f64 * t40760 + 0.25538759935978703638e-4_f64 * t40762 + 0.85129199786595678796e-5_f64 * t40764;
    (t40756, t40766)
}
