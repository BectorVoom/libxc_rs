//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1261/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1261<F: Float>(t124200: F, t446: F, t9770: F, t121914: F, t1882: F, t31033: F, t31025: F, t6109: F, t681: F, t5053: F, t6061: F, t1434: F, t193: F, t2506: F, t108335: F, t1131: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t124202 = t446 * t9770 * t124200;
    let t124205 = t446 * t9770 * t121914;
    let t124207 = t1882 * t31033;
    let t124210 = t6109 * t681 * t31025;
    let t124212 = t6061 * t5053;
    let t124215 = t1434 * t193 * t2506 * t124212;
    let t124219 = t89 * t193 * t108335 * t1131;
    (t124202, t124205, t124207, t124210, t124212, t124215, t124219)
}
