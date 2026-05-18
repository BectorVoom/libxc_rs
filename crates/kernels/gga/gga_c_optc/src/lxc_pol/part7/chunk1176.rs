//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1176/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1176<F: Float>(t2364: F, t7239: F, t7294: F, t212: F, t2263: F, t362: F, t508: F, t896: F, t769: F, t2640: F, t7470: F, t2643: F, t7266: F) -> (F, F, F, F, F, F) {
    let t24386 = t2364 * t7239;
    let t24388 = t2364 * t7294;
    let t24391 = F::new(1.0) / t212 / t2263;
    let t24392 = t24391 * t362;
    let t24407 = t508 * t896;
    let t24408 = t24407 * t769;
    let t24410 = t2640 * t24408 * t7470;
    let t24412 = t2643 * t7266;
    (t24386, t24388, t24391, t24392, t24410, t24412)
}
