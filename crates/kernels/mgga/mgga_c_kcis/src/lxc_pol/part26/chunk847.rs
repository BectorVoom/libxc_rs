//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 847/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk847<F: Float>(t11409: F, t16046: F, t16050: F, t16052: F, t16127: F, t16129: F, t16146: F, t16292: F, t16301: F, t21186: F, t21188: F, t21190: F, t21193: F, t21229: F, t21234: F, t21237: F, t21240: F, t21243: F, t21246: F, t21249: F, t21424: F, t21445: F) -> (F,) {
    let t21447 = -0.22076e0 * t16127 - 0.18396666666666666667e0 * t16129 - 0.40256666666666666668e0 * t16052 - 0.26837777777777777779e0 * t16046 - t16292 + 0.36793333333333333333e-1 * t16146 + 0.67094444444444444443e-1 * t21186 - 0.20128333333333333333e0 * t21188 + 0.18396666666666666667e-1 * t21190 - 0.301925e0 * t21193 + t21424 - 0.27595e-1 * t21229 - 0.13418888888888888889e0 * t11409 + t16301 - 0.40256666666666666668e0 * t16050 + 0.12077e1 * t21234 - 0.33547222222222222222e0 * t21237 + 0.80513333333333333332e0 * t21240 - 0.181155e1 * t21243 + 0.16557e0 * t21246 - 0.36793333333333333333e-1 * t21249 + t21445;
    (t21447,)
}
