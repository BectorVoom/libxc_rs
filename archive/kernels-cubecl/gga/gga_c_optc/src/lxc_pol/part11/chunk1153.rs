//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1153/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1153<F: Float>(t17219: F, t2721: F, t8152: F, t14339: F, t3884: F, t42111: F, t17169: F, t17125: F, t16988: F, t864: F, t7380: F, t857: F) -> (F, F, F, F, F, F, F) {
    let t51355 = t2721 * t8152 * t17219;
    let t51360 = t3884 * t42111 * t14339;
    let t51363 = t2721 * t8152 * t17169;
    let t51368 = t2721 * t8152 * t17125;
    let t51399 = t864 * t16988;
    let t51400 = t51399 * t7380;
    let t51450 = t857 * t16988;
    (t51355, t51360, t51363, t51368, t51399, t51400, t51450)
}
