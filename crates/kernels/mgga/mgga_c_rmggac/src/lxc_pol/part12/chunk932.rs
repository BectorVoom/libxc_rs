//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 932/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk932<F: Float>(t42023: F, t2160: F, t638: F, t8854: F, t131: F, t4999: F, t639: F, t71: F, t5055: F, t7769: F, t1341: F, t575: F, t7310: F, t7244: F, t8427: F, t2031: F, t28295: F, t36893: F, t40788: F, t40960: F, t41999: F, t42003: F, t42007: F, t42011: F, t4985: F, t5199: F, t665: F, t739: F, t7672: F, t8876: F, t903: F, t931: F) -> (F,) {
    let t42024 = 0.81300399444200075504e-3 * t42023;
    let t42026 = t638 * t2160 * t8854;
    let t42027 = 0.81300399444200075504e-3 * t42026;
    let t42032 = t638 * t639 * t71 * t4999 * t131;
    let t42034 = t5055 * t7769;
    let t42035 = 0.23948483403727617128e0 * t42034;
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42046 = 0.42564599893297839398e-5 * t41999 + 0.23942587439980034662e-4 * t42003 + 0.23942587439980034662e-4 * t42007 + 0.11971293719990017331e-4 * t42011 + 0.17961362552795712846e0 * t903 * t665 * t5199 + 0.59871208509319042821e-1 * t4985 * t7672 - 0.2363e1 * t931 * t8876 - 0.11974241701863808564e0 * t739 * t40788 - t42024 - t42027 - 0.19863479950205658386e-4 * t36893 + 0.15243824895787514157e-3 * t42032 - t42035 + 0.11974241701863808564e0 * t28295 * t2031 - 0.59871208509319042821e-1 * t739 * t40960 + 0.30487649791575028314e-3 * t42042 + 0.59590439850616975157e-4 * t42044;
    (t42046,)
}
