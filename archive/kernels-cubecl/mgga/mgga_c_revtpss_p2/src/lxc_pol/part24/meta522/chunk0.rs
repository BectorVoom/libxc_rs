//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1551/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1551<F: Float>(t1247: F, t24772: F, t3172: F, t20819: F, t5292: F, t17505: F, t20783: F, t1260: F, t24699: F, t21242: F, t5378: F, t1785: F, t21271: F) -> (F, F, F, F, F, F) {
    let t82553 = t1247 * t3172 * t24772;
    let t82555 = t20819 * t5292;
    let t82560 = t17505 * t20783;
    let t82565 = t24699 * t1260;
    let t82595 = t21242 * t5378;
    let t82597 = t1785 * t21271;
    (t82553, t82555, t82560, t82565, t82595, t82597)
}
