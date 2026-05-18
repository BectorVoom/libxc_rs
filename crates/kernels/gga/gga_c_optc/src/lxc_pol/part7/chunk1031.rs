//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1031/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1031<F: Float>(t1758: F, t534: F, t6433: F, t2: F, t41: F, t14: F, t209: F, t6567: F, t543: F, t6363: F, t133: F, t1765: F) -> (F, F, F, F, F, F) {
    let t22494 = t1758 * t1758;
    let t22497 = F::new(24.0) * t6433 * t22494 * t534;
    let t22502 = t2 * t41;
    let t22508 = F::new(1.0) / t14 / t22502 * t2 * t6567 * t209 / F::new(48.0);
    let t22510 = t6363 * t543;
    let t22512 = t1765 * t133;
    (t22494, t22497, t22502, t22508, t22510, t22512)
}
