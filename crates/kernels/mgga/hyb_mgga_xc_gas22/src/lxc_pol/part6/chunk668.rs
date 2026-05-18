//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 668/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk668<F: Float>(t3316: F, t811: F, t1347: F, t2183: F, t809: F, t2188: F, t1336: F, t2194: F, t791: F, t2167: F, t2198: F, t3300: F, t3311: F) -> (F, F, F, F, F, F, F) {
    let t3318 = F::new(1.0) * t3316 * t811;
    let t3320 = F::new(1.0) * t2183 * t1347;
    let t3321 = t1347 * t809;
    let t3323 = F::new(2.0) * t2188 * t3321;
    let t3324 = t2194 * t1336;
    let t3325 = t3324 * t791;
    let t3329 = t2198 - t2167 / F::new(3.0) - t3300 / F::new(3.0) + t3311;
    (t3318, t3320, t3321, t3323, t3324, t3325, t3329)
}
