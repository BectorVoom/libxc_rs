//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 93/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk93<F: Float>(t212: F, t228: F, t86: F, t95: F, t98: F, zeta_threshold: F) -> (F, F) {
    let t230 = 0.621814e-1 * t212 * t228;
    let t231 = 2.0 <= zeta_threshold;
    let t233 = piecewise3(t231, t86, 2.0 * t95);
    let t234 = 0.0 <= zeta_threshold;
    let t235 = piecewise3(t234, t86, 0.0);
    let t237 = (t233 + t235 - 2.0) * t98;
    (t230, t237)
}
