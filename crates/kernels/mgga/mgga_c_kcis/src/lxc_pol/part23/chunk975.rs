//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 975/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk975<F: Float>(t17933: F, t17959: F, t11832: F, t15847: F, t15851: F, t15854: F, t15858: F, t15863: F, t15868: F, t15872: F, t15876: F, t15881: F, t15885: F, t15890: F, t15894: F, t15896: F, t15901: F, t15905: F, t17739: F, t4315: F, t6193: F, t626: F) -> (F, F) {
    let t17960 = t17933 + t17959;
    let t17962 = t17739 - F::new(0.23214722222222222222e-2) * t15847 + F::new(0.69644166666666666664e-2) * t15851 - F::new(0.23214722222222222222e-2) * t15854 + F::new(0.61905925925925925924e-2) * t15858 + F::new(0.38691203703703703703e-3) * t15863 - F::new(0.30952962962962962962e-2) * t15868 + F::new(0.25794135802469135802e-2) * t15872 - F::new(0.15476481481481481481e-2) * t15876 + F::new(0.46429444444444444444e-2) * t15881 - F::new(0.38691203703703703704e-2) * t15885 + F::new(0.46429444444444444443e-2) * t15890 + F::new(0.66725e-1) * t6193 * t4315 - F::new(0.11607361111111111111e-2) * t11832 + F::new(0.23214722222222222222e-2) * t15894 - F::new(0.41270617283950617282e-2) * t15896 - F::new(0.11607361111111111111e-2) * t15901 - F::new(0.30952962962962962962e-2) * t15905 + t17960 * t626;
    (t17960, t17962)
}
