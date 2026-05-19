//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 55/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk55<F: Float>(t12: F, t24: F, t85: F, t87: F, t91: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t136 = t85 * t85;
    let t137 = t87 * t87;
    let t138 = piecewise3::<F>(t84, t136, t137);
    let t139 = t91 * t91;
    let t140 = piecewise3::<F>(t90, t136, t139);
    let t142 = t138 / F::new(2.0) + t140 / F::new(2.0);
    (t136, t137, t139, t142)
}
