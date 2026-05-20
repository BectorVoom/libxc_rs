//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2707/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2707<F: Float>(t3520: F, t6513: F, t3495: F, t3476: F, t6481: F, t20520: F, t3479: F, t3451: F, t20382: F, t3523: F, t12555: F, t6534: F) -> (F, F, F, F, F, F, F) {
    let t69359 = t6513 * t3520;
    let t69371 = t6513 * t3495;
    let t69376 = t6481 * t3476;
    let t69411 = t20520 * t3479;
    let t69488 = t6481 * t3451;
    let t69504 = t20382 * t3523;
    let t69511 = t6534 * t12555;
    (t69359, t69371, t69376, t69411, t69488, t69504, t69511)
}
