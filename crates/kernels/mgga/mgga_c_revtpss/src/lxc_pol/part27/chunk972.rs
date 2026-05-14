//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 972/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk972<F: Float>(t136: F, t243: F, t2371: F, t94: F, t3302: F, t471: F, t1214: F, t5464: F, t197: F, t531: F, t2013: F) -> (F, F, F, F, F, F, F) {
    let t14685 = t243 * t136;
    let t18163 = t94 * t2371;
    let t21471 = t3302 * t471;
    let t21472 = t21471 * t1214;
    let t21483 = t5464 * t1214;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t14685, t18163, t21471, t21472, t21483, t25081, t25082)
}
