//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2747/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2747<F: Float>(t215: F, t2722: F, t231: F, t268: F, t2798: F, t2645: F, t14545: F, t251: F, t4503: F, t860: F, t786: F, t10115: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t39583 = t215 * t2722;
    let t39586 = t2798 * t268 * t39583 * t231;
    let t39595 = t2798 * t268 * t215 * t2645 * t231;
    let t39597 = t14545 * t251;
    let t39608 = t4503 * t860;
    let t39609 = t786 * t39608;
    let t39624 = t10115 * t883;
    (t39583, t39586, t39595, t39597, t39608, t39609, t39624)
}
