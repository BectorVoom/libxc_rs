//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 579/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk579<F: Float>(t1250: F, t3588: F, t482: F, t1042: F, t3140: F, t460: F, t1242: F, t472: F) -> (F, F, F, F) {
    let t3590 = t482 * t3588 * t1250;
    let t3591 = t1042 * t3590;
    let t3594 = t460 * t3140;
    let t3596 = 1.0 / t1242 / t472;
    (t3590, t3591, t3594, t3596)
}
