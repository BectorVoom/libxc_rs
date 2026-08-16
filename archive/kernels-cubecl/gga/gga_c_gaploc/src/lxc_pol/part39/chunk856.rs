//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 856/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk856<F: Float>(t12161: F, t795: F, t1853: F, t3721: F, t12380: F, t455: F, t145: F, t459: F, t12385: F, t2281: F, t1246: F, t135: F, t4074: F, t458: F, t9105: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t39403 = t795 * t12161;
    let t39454 = t3721 * t1853;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    let t39632 = t9105 * t4074 * pi * t1246 * t135 * t458;
    (t39403, t39454, t39622, t39624, t39626, t39632)
}
