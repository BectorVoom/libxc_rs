//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 831/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk831<F: Float>(t20024: F, t458: F, t20046: F, t20049: F, t3020: F, t77: F, t534: F, t73777: F, t11262: F, t20031: F, t419: F, t20039: F, t7705: F, t173: F, t20076: F, t20065: F) -> (F, F, F, F, F, F, F, F) {
    let t73983 = t458 * t20024;
    let t73985 = t458 * t20046;
    let t74009 = t3020 * t77 * t20049;
    let t74034 = t534 * t73777;
    let t74068 = t419 * t11262 * t20031;
    let t74126 = t419 * t7705 * t20039;
    let t74143 = t419 * t173 * t20076;
    let t74148 = t419 * t173 * t20065;
    (t73983, t73985, t74009, t74034, t74068, t74126, t74143, t74148)
}
