//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1549/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1549<F: Float>(t20816: F, t5293: F, t24611: F, t3172: F, t3711: F, t24252: F, t300: F, t17529: F, t20786: F, t21102: F, t5265: F, t5274: F) -> (F, F, F, F, F, F) {
    let t82338 = t5293 * t20816;
    let t82351 = t3711 * t3172 * t24611;
    let t82389 = t300 * t24252;
    let t82434 = t17529 * t20786;
    let t82441 = t21102 * t5265;
    let t82457 = t5274 * t20816;
    (t82338, t82351, t82389, t82434, t82441, t82457)
}
