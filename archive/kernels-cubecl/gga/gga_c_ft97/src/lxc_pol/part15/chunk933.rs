//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 933/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk933<F: Float>(t1526: F, t42262: F, t5198: F, t1882: F, t20146: F, t1546: F, t20149: F, t89: F, t20134: F, t7780: F, t20157: F, t20104: F) -> (F, F, F, F, F, F) {
    let t72992 = t1526 * t42262 * t5198;
    let t73256 = t1882 * t20146;
    let t73259 = t89 * t1546 * t20149;
    let t73262 = t89 * t7780 * t20134;
    let t73276 = t89 * t1546 * t20157;
    let t73299 = t1882 * t20104;
    (t72992, t73256, t73259, t73262, t73276, t73299)
}
