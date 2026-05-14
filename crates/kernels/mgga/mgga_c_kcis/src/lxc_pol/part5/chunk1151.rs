//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1151/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1151<F: Float>(t11409: F, t16046: F, t16048: F, t16052: F, t16127: F, t16129: F, t16145: F, t16146: F, t16156: F, t21186: F, t21188: F, t21190: F, t21193: F, t21226: F, t21229: F, t21234: F, t21237: F, t21240: F, t21243: F, t21246: F, t21249: F, t21288: F) -> (F,) {
    let t21290 = -0.21908444444444444444e0 * t16127 - 0.18257037037037037037e0 * t16129 - 0.39862222222222222222e0 * t16052 - 0.26574814814814814815e0 * t16046 - t16145 + 0.36514074074074074073e-1 * t16146 + 0.66437037037037037037e-1 * t21186 - 0.19931111111111111111e0 * t21188 + 0.18257037037037037037e-1 * t21190 - 0.29896666666666666667e0 * t21193 + t21226 - 0.27385555555555555556e-1 * t21229 - 0.13287407407407407408e0 * t11409 + 0.13287407407407407407e0 * t16048 - t16156 + 0.11958666666666666667e1 * t21234 - 0.33218518518518518518e0 * t21237 + 0.79724444444444444444e0 * t21240 - 0.17938e1 * t21243 + 0.16431333333333333333e0 * t21246 - 0.36514074074074074075e-1 * t21249 + t21288;
    (t21290,)
}
