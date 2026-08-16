//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 809/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk809<F: Float>(t13200: F, t1841: F, t13182: F, t29439: F, t11083: F, t2558: F, t943: F, t13225: F, t731: F, t13176: F, t2549: F, t33232: F, t9647: F) -> (F, F, F, F, F, F) {
    let t43098 = t1841 * t13200;
    let t43100 = t29439 * t13182;
    let t43127 = t943 * t11083 * t2558;
    let t43139 = t731 * t13225;
    let t43196 = t2549 * t13176;
    let t43224 = t9647 * t33232 * t2558;
    (t43098, t43100, t43127, t43139, t43196, t43224)
}
