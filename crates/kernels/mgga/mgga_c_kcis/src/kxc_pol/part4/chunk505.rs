//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 505/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk505<F: Float>(t2526: F, t62: F, t755: F, t752: F, t2379: F, t2381: F, t2386: F, t2390: F, t2416: F, t2423: F, t2427: F, t2430: F, t2482: F, t2486: F, t2494: F, t688: F, t707: F, t82: F) -> (F, F, F, F) {
    let t2527 = t62 * t2526;
    let t2528 = t755 * t2527;
    let t2529 = t752 * t2528;
    let t2531 = t2379 * t82 - 0.13345e0 * t2381 * t707 + 0.890445125e-2 * t2386 * t2390 - 0.66725e-1 * t688 * t2416 + 0.66725e-1 * t688 * t2390 + 0.30952962962962962962e-1 * t2423 - 0.2653111111111111111e-1 * t2427 + 0.2653111111111111111e-1 * t2430 + 0.99491666666666666664e-2 * t2482 - 0.19898333333333333333e-1 * t2486 + 0.19898333333333333333e-1 * t2494 - 0.99491666666666666664e-2 * t2529;
    (t2527, t2528, t2529, t2531)
}
