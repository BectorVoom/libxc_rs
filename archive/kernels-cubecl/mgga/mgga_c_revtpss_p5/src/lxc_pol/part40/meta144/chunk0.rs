//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 677/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk677<F: Float>(t1026: F, t127: F, t371: F, t1025: F, t3075: F, t373: F, t372: F, t225: F, t3046: F) -> (F, F, F, F, F) {
    let t3215 = t371 * t127 * t1026;
    let t3216 = t1025 * t3215;
    let t3218 = t373 * t3075;
    let t3220 = t371 * t372 * t3218;
    let t3223 = t3046 * t225;
    (t3215, t3216, t3218, t3220, t3223)
}
