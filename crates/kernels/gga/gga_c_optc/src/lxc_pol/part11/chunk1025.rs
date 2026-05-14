//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1025/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1025<F: Float>(t16420: F, t6941: F, t16326: F, t22265: F, t16323: F, t6879: F, t16416: F, t654: F, t16412: F, t9686: F, t16398: F, t2030: F, t2024: F, t16370: F, t16443: F, t669: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48487 = t6941 * t16420;
    let t48526 = t22265 * t16326;
    let t48528 = t16323 * t6879;
    let t48555 = t654 * t16416;
    let t48559 = t9686 * t16412;
    let t48571 = t2030 * t16398;
    let t48577 = t16323 * t2024;
    let t48590 = t16370 * t2024;
    let t48629 = t16443 * t669;
    (t48487, t48526, t48528, t48555, t48559, t48571, t48577, t48590, t48629)
}
