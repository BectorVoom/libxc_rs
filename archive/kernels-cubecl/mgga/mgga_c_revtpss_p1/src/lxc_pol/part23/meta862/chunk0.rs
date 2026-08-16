//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2752/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2752<F: Float>(t2608: F, t512: F, t6800: F, t177: F, t21931: F, t762: F, t1320: F, t22193: F, t22461: F, t4147: F, t749: F, t22212: F, t2516: F) -> (F, F, F, F, F, F) {
    let t73350 = t512 * t6800 * t2608;
    let t73352 = t21931 * t177 * t762;
    let t73374 = t1320 * t22193;
    let t73407 = t22461 * t4147;
    let t73476 = t512 * t21931 * t749;
    let t73481 = t22212 * t2516;
    (t73350, t73352, t73374, t73407, t73476, t73481)
}
