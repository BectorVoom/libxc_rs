//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 519/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk519<F: Float>(t2258: F, t36: F, t70: F, t607: F, t627: F, t362: F, t41: F, t47: F, t2251: F, t48: F, t59: F, t60: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2259 = t36 * t2258;
    let t2260 = t2259 * t70;
    let t2263 = t607 * t627;
    let t2269 = F::cast_from(1.0_f64) / t41 / t362;
    let t2270 = sigma0 * t2269;
    let t2275 = F::cast_from(1.0_f64) / t47;
    let t2276 = t2275 * t2251;
    let t2279 = t48 * t2258;
    let t2282 = F::cast_from(1.0_f64) / t59;
    let t2283 = t2282 * t2251;
    let t2286 = t60 * t2258;
    (t2259, t2260, t2263, t2270, t2275, t2276, t2279, t2282, t2283, t2286)
}
