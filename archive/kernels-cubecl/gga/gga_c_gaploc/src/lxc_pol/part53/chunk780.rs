//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 780/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk780<F: Float>(t12030: F, t501: F, t161: F, t39048: F, t12161: F, t795: F, t1853: F, t3721: F, t12380: F, t455: F, t145: F, t459: F) -> (F, F, F, F, F, F) {
    let t39340 = t12030 * t501;
    let t39347 = t39048 * t161;
    let t39403 = t795 * t12161;
    let t39454 = t3721 * t1853;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    (t39340, t39347, t39403, t39454, t39622, t39624)
}
