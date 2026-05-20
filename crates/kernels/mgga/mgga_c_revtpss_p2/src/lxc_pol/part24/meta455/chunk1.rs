//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1423/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1423<F: Float>(t10535: F, t14523: F, t9285: F, t14946: F, t2710: F, t10111: F, t22: F, t4518: F, t231: F, t39698: F, t4494: F, t40921: F, t4496: F) -> (F, F, F, F, F) {
    let t51635 = t10535 * t14523 * t9285;
    let t51646 = t2710 * t14946 * t9285;
    let t51660 = t10111 * t4518 * t22;
    let t51676 = t39698 * t4494 * t231 * t22;
    let t51686 = t40921 * t4496;
    (t51635, t51646, t51660, t51676, t51686)
}
