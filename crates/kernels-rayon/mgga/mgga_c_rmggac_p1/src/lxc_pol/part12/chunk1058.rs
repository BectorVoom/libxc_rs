//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1058/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1058(t8876: f64, t942: f64, t4961: f64, t668: f64, t1971: f64, t3351: f64, t5194: f64, t880: f64, t235: f64, t2379: f64, t26093: f64, t289: f64, t36748: f64, t36753: f64, t36754: f64, t36756: f64, t36758: f64, t36797: f64, t36802: f64, t36804: f64, t36806: f64, t36809: f64, t36811: f64, t36814: f64, t41386: f64, t515: f64) -> f64 {
    let t41929 = 0.4726e1_f64 * t942 * t8876;
    let t41932 = t4961 * t668;
    let t41949 = t3351 * t1971 * t880 * t5194;
    let t41951 = -t41929 + 0.59871208509319042821e-1_f64 * t26093 * t2379 - 0.4726e1_f64 * t289 * t41932 - 0.30487649791575028314e-3_f64 * t36748 - t36753 - 0.30487649791575028314e-3_f64 * t36754 + 0.60975299583150056628e-3_f64 * t36756 + 0.96056421943322389208e-3_f64 * t36758 - t36797 + t36802 + 0.16260079888840015101e-2_f64 * t36804 + 0.19211284388664477842e-2_f64 * t36806 + 0.16260079888840015101e-2_f64 * t36809 + 0.19211284388664477842e-2_f64 * t36811 - 0.15243824895787514157e-3_f64 * t36814 - 0.19957069503106347607e-1_f64 * t235 * t515 * t41386 + 0.10215503974391481455e-3_f64 * t41949;
    t41951
}
