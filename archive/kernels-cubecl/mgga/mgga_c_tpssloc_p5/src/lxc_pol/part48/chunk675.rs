//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 675/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk675<F: Float>(t22479: F, t510: F, t652: F, t2303: F, t71: F, t33: F, t9228: F, t240: F, t2235: F, t608: F, t641: F, t645: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t22480 = t510 * t22479;
    let t22482 = F::cast_from(2.0_f64) * t652 * t22480;
    let t22489 = t71 * t2303;
    let t22493 = t9228 * t33;
    let t22510 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t240;
    let t22519 = t2235 * t608;
    let t22527 = t72 * t641 * t645;
    (t22480, t22482, t22489, t22493, t22510, t22519, t22527)
}
