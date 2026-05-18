//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 806/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk806<F: Float>(t10012: F, t2684: F, t2925: F, t9438: F, t3005: F, t9800: F, t9829: F, t13142: F, t7416: F, t13149: F, t2464: F, t825: F) -> (F, F, F, F) {
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44004 = t9800 * t3005 * t9829;
    let t44009 = t7416 * t13142;
    let t44045 = t825 * t2464 * t13149;
    (t44001, t44004, t44009, t44045)
}
