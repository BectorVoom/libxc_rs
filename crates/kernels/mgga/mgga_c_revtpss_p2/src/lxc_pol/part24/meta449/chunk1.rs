//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1413/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413<F: Float>(t10111: F, t22: F, t5759: F, t14159: F, t3964: F, t9285: F, t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t10115: F, t1894: F) -> (F, F, F, F, F) {
    let t49361 = t10111 * t5759 * t22;
    let t49432 = t3964 * t14159 * t9285;
    let t49468 = t9292 * t5600;
    let t49471 = t786 * t1893 * t4075;
    let t49474 = t10115 * t1894;
    (t49361, t49432, t49468, t49471, t49474)
}
