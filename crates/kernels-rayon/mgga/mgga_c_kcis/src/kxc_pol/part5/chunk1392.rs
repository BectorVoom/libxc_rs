//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1392/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1392(t11409: f64, t16046: f64, t16050: f64, t16052: f64, t16127: f64, t16129: f64, t16146: f64, t17847: f64, t17856: f64, t21186: f64, t21188: f64, t21190: f64, t21193: f64, t21229: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64, t21246: f64, t21249: f64, t22956: f64, t22977: f64) -> f64 {
    let t22979 = -0.27785333333333333334e0_f64 * t16127 - 0.23154444444444444445e0_f64 * t16129 - 0.68863333333333333332e0_f64 * t16052 - 0.45908888888888888888e0_f64 * t16046 - t17847 + 0.4630888888888888889e-1_f64 * t16146 + 0.11477222222222222222e0_f64 * t21186 - 0.34431666666666666667e0_f64 * t21188 + 0.23154444444444444445e-1_f64 * t21190 - 0.516475e0_f64 * t21193 + t22956 - 0.34731666666666666667e-1_f64 * t21229 - 0.22954444444444444444e0_f64 * t11409 + t17856 - 0.68863333333333333332e0_f64 * t16050 + 0.20659e1_f64 * t21234 - 0.57386111111111111112e0_f64 * t21237 + 0.13772666666666666667e1_f64 * t21240 - 0.309885e1_f64 * t21243 + 0.20839e0_f64 * t21246 - 0.46308888888888888889e-1_f64 * t21249 + t22977;
    t22979
}
