//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 694/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk694<F: Float>(t2254: F, t2546: F, t147: F, t2454: F, t1087: F, t786: F, t818: F, t2716: F, t918: F, t2669: F, t2492: F, t891: F) -> (F, F, F, F, F, F, F) {
    let t7708 = t2546 * t2254;
    let t7730 = t2454 * t147;
    let t7735 = t1087 * t786;
    let t7739 = t1087 * t818;
    let t7764 = t918 * t2716;
    let t7776 = t918 * t2669;
    let t7807 = t2492 * t891;
    (t7708, t7730, t7735, t7739, t7764, t7776, t7807)
}
