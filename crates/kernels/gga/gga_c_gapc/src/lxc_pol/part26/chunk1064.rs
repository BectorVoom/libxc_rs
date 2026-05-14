//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1064/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1064<F: Float>(t1036: F, t1463: F, t33597: F, t5462: F, t9388: F, t11508: F, t2993: F, t5392: F, t11434: F, t21049: F, t3021: F, t11387: F, t21053: F, t21054: F, t11365: F, t5285: F, t5703: F) -> (F, F, F, F, F) {
    let t35222 = t5462 * t33597 * t1036 * t1463 * t9388;
    let t35225 = t2993 * t11508 * t5392;
    let t35228 = t11434 * t3021 * t21049;
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    (t35222, t35225, t35228, t35231, t35234)
}
