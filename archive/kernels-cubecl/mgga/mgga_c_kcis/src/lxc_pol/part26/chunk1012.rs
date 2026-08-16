//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1012/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1012<F: Float>(t11409: F, t16046: F, t16050: F, t16052: F, t16127: F, t16129: F, t16146: F, t17847: F, t17856: F, t21186: F, t21188: F, t21190: F, t21193: F, t21229: F, t21234: F, t21237: F, t21240: F, t21243: F, t21246: F, t21249: F, t22956: F, t22977: F) -> F {
    let t22979 = -F::cast_from(0.27785333333333333334e0_f64) * t16127 - F::cast_from(0.23154444444444444445e0_f64) * t16129 - F::cast_from(0.68863333333333333332e0_f64) * t16052 - F::cast_from(0.45908888888888888888e0_f64) * t16046 - t17847 + F::cast_from(0.4630888888888888889e-1_f64) * t16146 + F::cast_from(0.11477222222222222222e0_f64) * t21186 - F::cast_from(0.34431666666666666667e0_f64) * t21188 + F::cast_from(0.23154444444444444445e-1_f64) * t21190 - F::cast_from(0.516475e0_f64) * t21193 + t22956 - F::cast_from(0.34731666666666666667e-1_f64) * t21229 - F::cast_from(0.22954444444444444444e0_f64) * t11409 + t17856 - F::cast_from(0.68863333333333333332e0_f64) * t16050 + F::cast_from(0.20659e1_f64) * t21234 - F::cast_from(0.57386111111111111112e0_f64) * t21237 + F::cast_from(0.13772666666666666667e1_f64) * t21240 - F::cast_from(0.309885e1_f64) * t21243 + F::cast_from(0.20839e0_f64) * t21246 - F::cast_from(0.46308888888888888889e-1_f64) * t21249 + t22977;
    t22979
}
