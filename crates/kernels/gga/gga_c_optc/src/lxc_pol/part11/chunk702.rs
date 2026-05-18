//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 702/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk702<F: Float>(t195: F, t287: F, t362: F, t357: F, t355: F, t2471: F, t261: F) -> (F, F, F, F) {
    let t7328 = t195 * t287;
    let t7329 = t7328 * t362;
    let t7330 = t357 * t7329;
    let t7332 = F::new(5.0) / F::new(27.0) * t355 * t7330;
    let t7341 = F::new(1.0) / t2471 / t261;
    (t7328, t7330, t7332, t7341)
}
