//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1382/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1382(t28426: f64, t8151: f64, t28397: f64, t1598: f64, t60780: f64, t102706: f64, t102709: f64, t102712: f64, t102715: f64, t102723: f64, t29393: f64, t7901: f64, t94651: f64, t98888: f64, t98903: f64) -> f64 {
    let t103736 = t8151 * t28426;
    let t103739 = t28397 * t28426;
    let t103744 = t60780 * t1598;
    let t103747 = 0.33163888888888888888e-2_f64 * t102706 - 0.22109259259259259259e-2_f64 * t102709 + 0.99491666666666666664e-2_f64 * t102712 + 0.13265555555555555555e-1_f64 * t102715 - 0.12356481481481481481e-2_f64 * t103736 + 0.15445601851851851852e-3_f64 * t94651 + 0.6183646701388888889e-4_f64 * t103739 + 0.22109259259259259259e-2_f64 * t102723 + t98888 + t98903 + 0.69505208333333333333e-3_f64 * t29393 * t7901 + 0.92754700520833333333e-4_f64 * t103744 * t7901;
    t103747
}
