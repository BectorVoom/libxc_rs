//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 509/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk509<F: Float>(t2258: F, t36: F, t70: F, t607: F, t627: F, t362: F, t41: F, t47: F, t2251: F, t48: F, t59: F, t60: F, t239: F, t64: F, t44: F, t49: F, t56: F, t614: F, t617: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2259 = t36 * t2258;
    let t2260 = t2259 * t70;
    let t2263 = t607 * t627;
    let t2269 = 1.0 / t41 / t362;
    let t2270 = sigma0 * t2269;
    let t2275 = 1.0 / t47;
    let t2276 = t2275 * t2251;
    let t2279 = t48 * t2258;
    let t2282 = 1.0 / t59;
    let t2283 = t2282 * t2251;
    let t2286 = t60 * t2258;
    let t2289 = t64 * t239;
    let t2290 = 88.0 / 9.0 * t2289;
    let t2291 = 88.0 / 9.0 * t2270 * t49 - 40.0 / 9.0 * t614 * t617 + 5.0 / 18.0 * t44 * t2276 + 5.0 / 6.0 * t44 * t2279 + 5.0 / 18.0 * t56 * t2283 - 5.0 / 6.0 * t56 * t2286 - t2290;
    (t2259, t2260, t2263, t2270, t2275, t2282, t2283, t2286, t2289, t2290, t2291)
}
