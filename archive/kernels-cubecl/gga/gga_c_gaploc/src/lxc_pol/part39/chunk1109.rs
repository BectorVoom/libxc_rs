//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1109/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1109<F: Float>(t2676: F, t39050: F, t2365: F, t39040: F, t6111: F, t12251: F, t2021: F, t7372: F, t12205: F, t2028: F, t2536: F, t787: F) -> (F, F, F, F) {
    let t47193 = t39050 * t2676;
    let t47196 = t6111 * t2365 * t39040;
    let t47199 = t2021 * t12251 * t7372;
    let t47203 = t787 * t2536 * t12205 * t2028;
    (t47193, t47196, t47199, t47203)
}
