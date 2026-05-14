//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1221/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1221<F: Float>(t3116: F, t3120: F, t3126: F, t8446: F, t449: F, t508: F, t3105: F, t3103: F, t3109: F, t3104: F, t3119: F, t8415: F, t8487: F, t3132: F, t4357: F, t24502: F, t465: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26908 = t3116 * t8446 * t3126 * t3120;
    let t26910 = t508 * t449;
    let t26911 = t26910 * t3105;
    let t26913 = t3103 * t26911 * t3109;
    let t26915 = t3126 * t3126;
    let t26916 = t3104 * t26915;
    let t26929 = t3119 * t8415;
    let t26936 = t8487 * t3126;
    let t26938 = t3132 * t26936 * t4357;
    let t26940 = t465 * t24502;
    (t26908, t26911, t26913, t26915, t26916, t26929, t26936, t26938, t26940)
}
