//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 538/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk538<F: Float>(t3204: F, t366: F, t3059: F, t373: F, t371: F, t372: F, t1024: F, t1053: F, t1026: F, t127: F, t1025: F, t3075: F, t225: F, t3046: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3205 = t3204 * t366;
    let t3206 = t373 * t3059;
    let t3208 = t371 * t372 * t3206;
    let t3211 = t1024 * t1053;
    let t3215 = t371 * t127 * t1026;
    let t3216 = t1025 * t3215;
    let t3218 = t373 * t3075;
    let t3220 = t371 * t372 * t3218;
    let t3223 = t3046 * t225;
    (t3205, t3206, t3208, t3211, t3215, t3216, t3218, t3220, t3223)
}
