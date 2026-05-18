//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 769/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk769<F: Float>(t12030: F, t501: F, t161: F, t39048: F, t12161: F, t795: F, t12380: F, t455: F, t145: F, t459: F, t12385: F, t2281: F) -> (F, F, F, F, F, F) {
    let t39340 = t12030 * t501;
    let t39347 = t39048 * t161;
    let t39403 = t795 * t12161;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    (t39340, t39347, t39403, t39622, t39624, t39626)
}
