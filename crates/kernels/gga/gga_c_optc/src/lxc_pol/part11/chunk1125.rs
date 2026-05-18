//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1125/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1125<F: Float>(t16438: F, t654: F, t127: F, t16370: F, t16394: F, t2030: F, t16382: F, t16406: F, t16402: F, t16386: F, t6799: F, t16323: F) -> (F, F, F, F, F, F, F, F) {
    let t48272 = t654 * t16438;
    let t48308 = t16370 * t127;
    let t48313 = t2030 * t16394;
    let t48315 = t2030 * t16382;
    let t48317 = t2030 * t16406;
    let t48320 = t2030 * t16402;
    let t48356 = t6799 * t16386;
    let t48365 = t16323 * t127;
    (t48272, t48308, t48313, t48315, t48317, t48320, t48356, t48365)
}
