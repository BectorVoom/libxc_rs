//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 888/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk888<F: Float>(t26560: F, t689: F, t7399: F, t786: F, t789: F, t2062: F, t2453: F, t2458: F, t2411: F, t7427: F) -> (F, F, F, F) {
    let t26561 = t689 * t26560;
    let t26563 = t786 * t7399;
    let t26564 = t26563 * t789;
    let t26576 = t2453 * t2062;
    let t26578 = F::new(0.11565819519348392139e-2) * t26576 * t2458;
    let t26585 = t7427 * t2411;
    (t26561, t26564, t26578, t26585)
}
