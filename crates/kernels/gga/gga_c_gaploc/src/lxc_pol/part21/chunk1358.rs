//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1358/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1358<F: Float>(t10292: F, t12034: F, t12031: F, t12149: F, t12327: F, t1352: F, t3690: F, t3689: F, t447: F, t2366: F, t475: F, t6508: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35257 = F::cast_from(2.0_f64) * t10292;
    let t38262 = F::cast_from(2.0_f64) * t12034;
    let t38263 = F::cast_from(2.0_f64) * t12031;
    let t38264 = F::cast_from(2.0_f64) * t12149;
    let t38266 = F::cast_from(2.0_f64) * t12327;
    let t38267 = t3690 * t1352;
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    let t38277 = t6508 * t38276;
    (t35257, t38262, t38263, t38264, t38266, t38267, t38271, t38272, t38276, t38277)
}
