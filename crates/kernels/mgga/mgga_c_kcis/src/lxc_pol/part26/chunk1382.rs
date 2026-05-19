//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1382/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1382<F: Float>(t28426: F, t8151: F, t28397: F, t1598: F, t60780: F, t102706: F, t102709: F, t102712: F, t102715: F, t102723: F, t29393: F, t7901: F, t94651: F, t98888: F, t98903: F) -> F {
    let t103736 = t8151 * t28426;
    let t103739 = t28397 * t28426;
    let t103744 = t60780 * t1598;
    let t103747 = F::cast_from(0.33163888888888888888e-2_f64) * t102706 - F::cast_from(0.22109259259259259259e-2_f64) * t102709 + F::cast_from(0.99491666666666666664e-2_f64) * t102712 + F::cast_from(0.13265555555555555555e-1_f64) * t102715 - F::cast_from(0.12356481481481481481e-2_f64) * t103736 + F::cast_from(0.15445601851851851852e-3_f64) * t94651 + F::cast_from(0.6183646701388888889e-4_f64) * t103739 + F::cast_from(0.22109259259259259259e-2_f64) * t102723 + t98888 + t98903 + F::cast_from(0.69505208333333333333e-3_f64) * t29393 * t7901 + F::cast_from(0.92754700520833333333e-4_f64) * t103744 * t7901;
    t103747
}
