//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 857/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk857<F: Float>(t21577: F, t458: F, t21570: F, t1775: F, t21589: F, t21603: F, t21585: F, t21435: F, t2336: F, t89: F, t21420: F, t681: F, t21412: F, t21400: F, t375: F, t21431: F, t41962: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t81040 = t458 * t21577;
    let t81042 = t458 * t21570;
    let t81048 = t1775 * t21589;
    let t81050 = t1775 * t21603;
    let t81057 = t1775 * t21585;
    let t81095 = t89 * t2336 * t21435;
    let t81102 = t89 * t681 * t21420;
    let t81105 = t89 * t2336 * t21412;
    let t81124 = t89 * t375 * t21400;
    let t81131 = t89 * t41962 * t21431;
    (t81040, t81042, t81048, t81050, t81057, t81095, t81102, t81105, t81124, t81131)
}
