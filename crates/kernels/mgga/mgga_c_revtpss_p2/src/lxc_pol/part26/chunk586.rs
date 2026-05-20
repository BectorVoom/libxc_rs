//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 586/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk586<F: Float>(t3584: F, t482: F, t371: F, t372: F, t225: F, t3555: F, t480: F, t3566: F) -> (F, F, F, F, F) {
    let t3661 = t482 * t3584;
    let t3663 = t371 * t372 * t3661;
    let t3666 = t3555 * t225;
    let t3667 = t3666 * t480;
    let t3670 = t3566 * t225;
    (t3661, t3663, t3666, t3667, t3670)
}
