//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2188/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2188<F: Float>(t30123: F, t95088: F, t670: F, t7724: F, t1353: F, t6922: F, t25082: F, t8717: F, t30088: F, t689: F, t25904: F, t25899: F) -> (F, F, F, F, F) {
    let t108117 = F::new(6.0) * t95088 * t30123;
    let t108120 = t7724 * t670;
    let t108126 = t6922 * t1353;
    let t108129 = F::new(3.0) * t25082 * t8717 * t108126;
    let t108132 = t30088 * t689;
    let t108133 = t25904 * t108132;
    let t108135 = t25899 * t108132;
    (t108117, t108120, t108129, t108133, t108135)
}
