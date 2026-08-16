//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1065/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1065(t5055: f64, t7769: f64, t1341: f64, t575: f64, t638: f64, t7310: f64, t7244: f64, t8427: f64, t2031: f64, t28295: f64, t36893: f64, t40788: f64, t40960: f64, t41999: f64, t42003: f64, t42007: f64, t42011: f64, t42024: f64, t42027: f64, t42032: f64, t4985: f64, t5199: f64, t665: f64, t739: f64, t7672: f64, t8876: f64, t903: f64, t931: f64) -> f64 {
    let t42034 = t5055 * t7769;
    let t42035 = 0.23948483403727617128e0_f64 * t42034;
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42046 = 0.42564599893297839398e-5_f64 * t41999 + 0.23942587439980034662e-4_f64 * t42003 + 0.23942587439980034662e-4_f64 * t42007 + 0.11971293719990017331e-4_f64 * t42011 + 0.17961362552795712846e0_f64 * t903 * t665 * t5199 + 0.59871208509319042821e-1_f64 * t4985 * t7672 - 0.2363e1_f64 * t931 * t8876 - 0.11974241701863808564e0_f64 * t739 * t40788 - t42024 - t42027 - 0.19863479950205658386e-4_f64 * t36893 + 0.15243824895787514157e-3_f64 * t42032 - t42035 + 0.11974241701863808564e0_f64 * t28295 * t2031 - 0.59871208509319042821e-1_f64 * t739 * t40960 + 0.30487649791575028314e-3_f64 * t42042 + 0.59590439850616975157e-4_f64 * t42044;
    t42046
}
