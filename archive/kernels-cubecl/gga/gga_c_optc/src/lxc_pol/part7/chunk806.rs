//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 806/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk806<F: Float>(t7592: F, t7523: F, t2284: F, t7359: F, t25: F) -> (F, F, F, F) {
    let t7593 = F::cast_from(0.36793333333333333333e0_f64) * t7592;
    let t7594 = F::cast_from(0.93932222222222222223e0_f64) * t7523;
    let t7595 = t2284 * t7359;
    let t7596 = t25 * t7595;
    (t7593, t7594, t7595, t7596)
}
