//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2476/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476<F: Float>(t2439: F, t3421: F, t12278: F, t698: F, t12274: F, t25273: F, t268: F, t404: F) -> (F, F, F, F) {
    let t43783 = t2439 * t3421;
    let t43785 = t698 * t12278;
    let t43787 = t698 * t12274;
    let t43813 = t268 * t25273 * t404;
    (t43783, t43785, t43787, t43813)
}
