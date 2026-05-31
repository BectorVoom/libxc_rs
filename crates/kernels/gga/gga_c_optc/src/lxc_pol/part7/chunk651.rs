//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 651/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk651<F: Float>(t50: F, t1900: F, t104: F, t95: F, t176: F, t185: F, t102: F, t108: F, t110: F, t115: F, t56: F, t5: F, t1: F, t2060: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t3298 = piecewise3::<F>(t51, F::cast_from(0.0_f64), t1900);
    let t3308 = t95 * t104;
    let t3313 = t176 * t185;
    let t3314 = t102 * t108;
    let t3315 = t3314 * t110;
    let t3316 = t3313 * t3315;
    let t3317 = t56 * t115;
    let t3318 = t3317 * t5;
    let t3411 = t2060 * t1;
    (t3298, t3308, t3313, t3314, t3315, t3316, t3318, t3411)
}
