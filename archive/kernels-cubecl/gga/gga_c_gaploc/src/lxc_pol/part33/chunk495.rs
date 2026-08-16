//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 495/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk495<F: Float>(t2541: F, t740: F, t731: F, t945: F, t679: F, t78: F, t278: F, t481: F) -> (F, F, F, F) {
    let t2542 = t2541 * t740;
    let t2545 = t731 * t945;
    let t2547 = t78 * t679;
    let t2549 = t481 * t2547 * t278;
    (t2542, t2545, t2547, t2549)
}
