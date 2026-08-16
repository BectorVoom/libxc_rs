//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1071/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1071<F: Float>(t378: F, t6235: F, t1678: F, t4746: F, t6343: F, t994: F, t19462: F, t6461: F, t698: F, t6464: F, t6467: F, t6422: F, t689: F) -> (F, F, F, F, F, F, F, F) {
    let t20178 = t6235 * t378;
    let t20191 = t4746 * t1678;
    let t20204 = t994 * t6343;
    let t20211 = t19462 * t378;
    let t20276 = t698 * t6461;
    let t20278 = t698 * t6464;
    let t20280 = t698 * t6467;
    let t20283 = t689 * t6422;
    (t20178, t20191, t20204, t20211, t20276, t20278, t20280, t20283)
}
