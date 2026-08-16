//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1464/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464<F: Float>(t1063: F, t11986: F, t247: F, t6096: F, t1086: F, t6343: F, t994: F, t19462: F, t3286: F, t3298: F, t6235: F, t3316: F) -> (F, F, F, F, F) {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67652 = t994 * t1086 * t6343;
    let t67714 = t19462 * t3286;
    let t67725 = t6235 * t3298;
    let t67790 = t6235 * t3316;
    (t67575, t67652, t67714, t67725, t67790)
}
