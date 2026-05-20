//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3656/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3656<F: Float>(t422: F, t69044: F, t69058: F, t69072: F, t69086: F, t5104: F, t3433: F, t3435: F, t1150: F, t3384: F, t16835: F, t5105: F) -> (F, F, F, F) {
    let t69090 = F::new(0.621814e-1) * (t69044 + t69058 + t69072 + t69086) * t422;
    let t69091 = t5104 * t5104;
    let t69094 = F::cast_from(0.32163958997385070134e2_f64) * t3433 * t69091 * t3435;
    let t69097 = F::new(4.0) * t3384 * t69091 * t1150;
    let t69099 = F::new(4.0) * t16835 * t5105;
    (t69090, t69094, t69097, t69099)
}
