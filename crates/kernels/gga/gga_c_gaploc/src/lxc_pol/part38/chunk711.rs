//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 711/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk711<F: Float>(t13538: F, t2580: F, t2558: F, t3641: F, t943: F, t3650: F, t948: F, t2508: F, t3645: F, t11622: F, t2562: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t13539 = t2580 * t13538;
    let t13542 = t3641 * t2558;
    let t13543 = t943 * t13542;
    let t13544 = F::new(0.32043859292259267849e-3) * t13543;
    let t13545 = t3650 * t948;
    let t13547 = F::new(0.53833683610995569986e-1) * t2508 * t13545;
    let t13548 = t3645 * t2558;
    let t13549 = t943 * t13548;
    let t13550 = F::new(0.32043859292259267849e-3) * t13549;
    let t13552 = t2562 * t883 * t11622;
    (t13539, t13542, t13544, t13545, t13547, t13548, t13550, t13552)
}
