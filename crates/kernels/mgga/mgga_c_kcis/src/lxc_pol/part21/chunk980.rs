//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 980/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk980<F: Float>(t2175: F, t26703: F, t26823: F, t26826: F, t26829: F, t26834: F, t26837: F, t26838: F, t26841: F, t26844: F, t26846: F, t26849: F, t26852: F, t26856: F, t26857: F, t26860: F, t7703: F) -> (F,) {
    let t26864 = -0.69505208333333333333e-3 * t26823 * t2175 + 0.33163888888888888888e-2 * t26826 + 0.24872916666666666666e-2 * t26829 + 0.24320185185185185185e-1 * t26834 - t26837 - 0.88437037037037037034e-2 * t26838 - 0.88437037037037037034e-2 * t26841 + 0.16581944444444444444e-2 * t26844 - 0.33163888888888888888e-2 * t26846 + 0.33163888888888888888e-2 * t26849 - 0.13265555555555555555e-1 * t26852 + t26856 + 0.37069444444444444444e-2 * t26857 * t2175 - 0.46336805555555555556e-3 * t26860 + 0.46336805555555555556e-3 * t7703 * t26703;
    (t26864,)
}
