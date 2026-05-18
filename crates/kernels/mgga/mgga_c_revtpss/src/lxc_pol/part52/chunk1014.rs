//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1014/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1014<F: Float>(t31756: F, t4364: F, t837: F, t31755: F, t1955: F, t843: F, t8464: F, t8468: F, t233: F, t239: F, t240: F, t31752: F) -> (F, F, F, F) {
    let t31758 = t4364 * t31756 * t837;
    let t31759 = t31755 * t31758;
    let t31763 = t1955 * t8464 * t843 * t8468;
    let t31767 = t31752 * t233 * t239 * t240;
    (t31758, t31759, t31763, t31767)
}
