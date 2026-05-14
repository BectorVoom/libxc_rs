//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 961/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk961<F: Float>(t26140: F, t8428: F, t3016: F, t375: F, t3019: F, t3057: F, t3060: F, t2915: F, t2991: F, t2848: F, t2854: F, t209: F, t2139: F, t371: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26141 = t26140 * t8428;
    let t26193 = t3016 * t3016;
    let t26195 = t375 / t26193;
    let t26196 = t3019 * t3019;
    let t26197 = 1.0 / t26196;
    let t26213 = t3057 * t3057;
    let t26214 = 1.0 / t26213;
    let t26216 = t3060 * t3060;
    let t26217 = 1.0 / t26216;
    let t26224 = 1.0 / t3057 / t2915;
    let t26248 = t375 / t3016 / t2991;
    let t26255 = 1.0 / t2848 / t2854;
    let t26261 = t209 * t2139 * t371;
    (t26141, t26195, t26197, t26214, t26217, t26224, t26248, t26255, t26261)
}
