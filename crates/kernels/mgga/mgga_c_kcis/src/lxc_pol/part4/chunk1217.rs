//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1217/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1217<F: Float>(t14607: F, t10477: F, t14619: F, t14624: F, t14631: F, t14635: F, t14638: F, t14642: F, t14644: F, t14647: F, t14652: F, t15469: F, t430: F) -> (F, F) {
    let t15671 = F::new(0.15476481481481481481e-2) * t14607;
    let t15686 = -F::new(0.10446625e-1) * t14619 - F::new(0.18571777777777777777e-1) * t14624 + t15469 * t430 + F::new(0.46429444444444444443e-2) * t14631 - F::new(0.15476481481481481481e-2) * t14635 - F::new(0.15476481481481481481e-2) * t10477 - F::new(0.23214722222222222222e-2) * t14638 + F::new(0.69644166666666666666e-2) * t14642 + F::new(0.15476481481481481481e-2) * t14644 - F::new(0.41270617283950617284e-2) * t14647 + F::new(0.77382407407407407406e-3) * t14652;
    (t15671, t15686)
}
