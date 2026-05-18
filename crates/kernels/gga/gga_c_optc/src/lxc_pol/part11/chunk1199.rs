//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1199/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1199<F: Float>(t18019: F, t3234: F, t9189: F, t1178: F, t54599: F, t18030: F, t12068: F, t18012: F, t1107: F, t17697: F, t9122: F, t9124: F) -> (F, F, F, F, F, F) {
    let t55210 = t3234 * t9189 * t18019;
    let t55214 = t1178 * t54599;
    let t55262 = t3234 * t9189 * t18030;
    let t55265 = t3234 * t12068 * t18012;
    let t55330 = t1107 * t17697;
    let t55332 = t9122 * t55330 * t9124;
    (t55210, t55214, t55262, t55265, t55330, t55332)
}
