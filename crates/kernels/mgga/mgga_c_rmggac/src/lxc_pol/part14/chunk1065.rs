//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1065/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1065<F: Float>(t5055: F, t7769: F, t1341: F, t575: F, t638: F, t7310: F, t7244: F, t8427: F, t2031: F, t28295: F, t36893: F, t40788: F, t40960: F, t41999: F, t42003: F, t42007: F, t42011: F, t42024: F, t42027: F, t42032: F, t4985: F, t5199: F, t665: F, t739: F, t7672: F, t8876: F, t903: F, t931: F) -> F {
    let t42034 = t5055 * t7769;
    let t42035 = F::cast_from(0.23948483403727617128e0_f64) * t42034;
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42046 = F::cast_from(0.42564599893297839398e-5_f64) * t41999 + F::cast_from(0.23942587439980034662e-4_f64) * t42003 + F::cast_from(0.23942587439980034662e-4_f64) * t42007 + F::cast_from(0.11971293719990017331e-4_f64) * t42011 + F::cast_from(0.17961362552795712846e0_f64) * t903 * t665 * t5199 + F::cast_from(0.59871208509319042821e-1_f64) * t4985 * t7672 - F::new(0.2363e1) * t931 * t8876 - F::cast_from(0.11974241701863808564e0_f64) * t739 * t40788 - t42024 - t42027 - F::cast_from(0.19863479950205658386e-4_f64) * t36893 + F::cast_from(0.15243824895787514157e-3_f64) * t42032 - t42035 + F::cast_from(0.11974241701863808564e0_f64) * t28295 * t2031 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t40960 + F::cast_from(0.30487649791575028314e-3_f64) * t42042 + F::cast_from(0.59590439850616975157e-4_f64) * t42044;
    t42046
}
