//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 797/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk797<F: Float>(t12506: F, t1429: F, t549: F, t2492: F, t9267: F, t9278: F, t2482: F, t3133: F, t3125: F, t9263: F, t1538: F, t30208: F, t6583: F, t883: F) -> (F, F, F, F, F) {
    let t40283 = t1429 * t549 * t12506;
    let t40301 = t9267 * t2492 * t9278;
    let t40320 = t9267 * t3133 * t2482;
    let t40332 = t9263 * t3125 * t2482;
    let t40336 = t6583 * t1538 * t883 * t30208;
    (t40283, t40301, t40320, t40332, t40336)
}
