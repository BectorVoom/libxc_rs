//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 616/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk616<F: Float>(t3284: F, t5101: F, t914: F, t2847: F, t11: F) -> (F, F, F, F) {
    let t5102 = t3284 * t5101;
    let t5103 = t914 * t5102;
    let t5107 = t2847 * t5101;
    let t5108 = t11 * t5107;
    (t5102, t5103, t5107, t5108)
}
