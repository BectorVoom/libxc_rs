//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1154/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1154<F: Float>(t51450: F, t8214: F, t8216: F, t8208: F, t8210: F, t10856: F, t2668: F, t4963: F, t17125: F, t2812: F, t8143: F, t50765: F, t953: F) -> (F, F, F, F, F) {
    let t51452 = t8214 * t51450 * t8216;
    let t51461 = t8208 * t51450 * t8210;
    let t51502 = t2668 * t10856 * t4963;
    let t51515 = t2812 * t8143 * t17125;
    let t51564 = t953 * t50765;
    (t51452, t51461, t51502, t51515, t51564)
}
