//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2805/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2805<F: Float>(t40270: F, t4496: F, t136: F, t137: F, t14597: F, t2438: F, t2723: F, t49180: F, t836: F, t2457: F, t2710: F, t4469: F) -> (F, F, F) {
    let t51553 = t40270 * t4496;
    let t51560 = t49180 * t14597 * t2723 * t136 * t137 * t2438 * t836;
    let t51561 = F::cast_from(0.69394917116090352834e-2_f64) * t51560;
    let t51564 = t2710 * t4469 * t136 * t2457;
    (t51553, t51561, t51564)
}
