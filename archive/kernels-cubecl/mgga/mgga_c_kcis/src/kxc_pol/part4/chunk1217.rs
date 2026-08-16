//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1217/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1217<F: Float>(t14607: F, t10477: F, t14619: F, t14624: F, t14631: F, t14635: F, t14638: F, t14642: F, t14644: F, t14647: F, t14652: F, t15469: F, t430: F) -> (F, F) {
    let t15671 = F::cast_from(0.15476481481481481481e-2_f64) * t14607;
    let t15686 = -F::cast_from(0.10446625e-1_f64) * t14619 - F::cast_from(0.18571777777777777777e-1_f64) * t14624 + t15469 * t430 + F::cast_from(0.46429444444444444443e-2_f64) * t14631 - F::cast_from(0.15476481481481481481e-2_f64) * t14635 - F::cast_from(0.15476481481481481481e-2_f64) * t10477 - F::cast_from(0.23214722222222222222e-2_f64) * t14638 + F::cast_from(0.69644166666666666666e-2_f64) * t14642 + F::cast_from(0.15476481481481481481e-2_f64) * t14644 - F::cast_from(0.41270617283950617284e-2_f64) * t14647 + F::cast_from(0.77382407407407407406e-3_f64) * t14652;
    (t15671, t15686)
}
