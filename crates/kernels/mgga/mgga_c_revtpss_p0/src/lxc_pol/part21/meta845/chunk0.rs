//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3162/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3162<F: Float>(t3476: F, t5117: F, t12228: F, t16668: F, t44017: F, t16661: F, t3427: F, t3433: F, t12243: F, t16688: F, t3385: F, t5105: F) -> (F, F, F, F, F) {
    let t58317 = t5117 * t3476;
    let t58322 = F::cast_from(0.62071215503128080361e4_f64) * t44017 * t16668 * t12228;
    let t58325 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t16661 * t3427;
    let t58327 = F::cast_from(18.0_f64) * t12243 * t16688;
    let t58330 = F::cast_from(18.0_f64) * t3433 * t5105 * t3385;
    (t58317, t58322, t58325, t58327, t58330)
}
