//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 496/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk496<F: Float>(t3086: F, t496: F, t1: F, t1244: F, t598: F, t104: F, t95: F, t176: F, t185: F, t102: F, t108: F, t110: F) -> (F, F, F, F, F, F, F) {
    let t3284 = t3086 * t496;
    let t3305 = t1244 * t1;
    let t3306 = t3305 * t598;
    let t3308 = t95 * t104;
    let t3313 = t176 * t185;
    let t3314 = t102 * t108;
    let t3315 = t3314 * t110;
    (t3284, t3305, t3306, t3308, t3313, t3314, t3315)
}
