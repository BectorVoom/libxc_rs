//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1131/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1131<F: Float>(t2453: F, t26264: F, t7496: F, t9692: F, t26249: F, t9664: F, t94701: F, t96204: F, t26359: F, t9303: F, t1892: F, t786: F) -> (F, F, F, F, F, F) {
    let t96515 = t2453 * t26264;
    let t96549 = F::new(0.30356481678079769392e-1) * t7496 * t9692;
    let t96564 = F::new(0.46263278077393568556e-2) * t26249 * t9664;
    let t96584 = F::new(0.51727911450665971904e-3) * t94701 * t96204;
    let t96591 = F::new(0.26019841438354088051e-2) * t9303 * t26359;
    let t97699 = t786 * t1892;
    (t96515, t96549, t96564, t96584, t96591, t97699)
}
