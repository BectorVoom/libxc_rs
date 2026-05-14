//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 580/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk580<F: Float>(t3596: F, t474: F, t3147: F, t479: F, t3594: F, t1248: F) -> (F, F, F, F, F) {
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    let t3599 = t3597 * t3598;
    let t3600 = t3594 * t3599;
    let t3601 = t1248 * t1248;
    (t3597, t3598, t3599, t3600, t3601)
}
