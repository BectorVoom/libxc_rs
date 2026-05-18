//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 532/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk532<F: Float>(t151: F, t3328: F, t947: F, t377: F, t941: F, t322: F, t839: F, t1089: F, t175: F, t384: F, t301: F, t864: F) -> (F, F, F, F, F, F) {
    let t3329 = t151 * t3328;
    let t3330 = t3329 * t947;
    let t3343 = t377 * t941;
    let t3344 = t3343 * t947;
    let t3346 = t839 * t322;
    let t3348 = t1089 * t175 * t3346;
    let t3349 = t384 * t3348;
    let t3355 = t864 * t301;
    (t3330, t3344, t3346, t3348, t3349, t3355)
}
