//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1386/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1386(t17933: f64, t17959: f64, t11832: f64, t15847: f64, t15851: f64, t15854: f64, t15858: f64, t15863: f64, t15868: f64, t15872: f64, t15876: f64, t15881: f64, t15885: f64, t15890: f64, t15894: f64, t15896: f64, t15901: f64, t15905: f64, t17739: f64, t4315: f64, t6193: f64, t626: f64) -> (f64, f64) {
    let t17960 = t17933 + t17959;
    let t17962 = t17739 - 0.23214722222222222222e-2_f64 * t15847 + 0.69644166666666666664e-2_f64 * t15851 - 0.23214722222222222222e-2_f64 * t15854 + 0.61905925925925925924e-2_f64 * t15858 + 0.38691203703703703703e-3_f64 * t15863 - 0.30952962962962962962e-2_f64 * t15868 + 0.25794135802469135802e-2_f64 * t15872 - 0.15476481481481481481e-2_f64 * t15876 + 0.46429444444444444444e-2_f64 * t15881 - 0.38691203703703703704e-2_f64 * t15885 + 0.46429444444444444443e-2_f64 * t15890 + 0.66725e-1_f64 * t6193 * t4315 - 0.11607361111111111111e-2_f64 * t11832 + 0.23214722222222222222e-2_f64 * t15894 - 0.41270617283950617282e-2_f64 * t15896 - 0.11607361111111111111e-2_f64 * t15901 - 0.30952962962962962962e-2_f64 * t15905 + t17960 * t626;
    (t17960, t17962)
}
