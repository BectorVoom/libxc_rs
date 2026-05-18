//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 980/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk980<F: Float>(t275: F, t9064: F, t4928: F, t645: F, t903: F, t1679: F, t7197: F, t7200: F, t38530: F, t7484: F, t7450: F, t10792: F, t1288: F, t2376: F, t2405: F, t26291: F, t27102: F, t36464: F, t40716: F, t40719: F, t40721: F, t40724: F, t40725: F, t40732: F, t40736: F, t40740: F, t40747: F, t5928: F, t72: F, t739: F, t7574: F, t7703: F) -> (F, F) {
    let t40750 = F::new(2.0) * t275 * t9064;
    let t40756 = t645 * t4928;
    let t40757 = t903 * t40756;
    let t40759 = t1679 * t7197;
    let t40760 = t40759 * t7200;
    let t40762 = t38530 * t7484;
    let t40764 = t38530 * t7450;
    let t40766 = t40716 - F::new(0.43368970657079495312e-4) * t40719 - F::new(0.71845450211182851384e0) * t26291 * t40721 - F::new(0.71845450211182851384e0) * t40724 * t40725 - F::new(0.35922725105591425692e0) * t739 * t7703 * t27102 + F::new(0.17025839957319135759e-4) * t40732 - F::new(0.20455996240684006296e-1) * t40736 + F::new(0.27274661654245341728e-1) * t40740 - F::new(0.59871208509319042821e-1) * t10792 * t2376 + F::new(0.85129199786595678796e-5) * t40747 + t40750 - F::new(0.24829349937757072982e-4) * t36464 + F::new(0.39914139006212695214e-1) * t5928 * t7574 + t72 * t1288 * t2405 + F::new(0.44903406381989282115e-1) * t40757 - F::new(0.81823984962736025184e-1) * t40760 + F::new(0.25538759935978703638e-4) * t40762 + F::new(0.85129199786595678796e-5) * t40764;
    (t40756, t40766)
}
