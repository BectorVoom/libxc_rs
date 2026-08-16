//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2634/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2634<F: Float>(t13921: F, t221: F, t4018: F, t4019: F, t2661: F, t3924: F, t3992: F, t5651: F, t5608: F, t1882: F, t4010: F, t9956: F) -> (F, F, F, F) {
    let t48445 = t4018 * t4019 * t221 * t13921;
    let t48449 = t2661 * t3992 * t5651 * t3924;
    let t48453 = t2661 * t3992 * t5608 * t3924;
    let t48455 = t4010 * t1882;
    let t48458 = t2661 * t3992 * t48455 * t9956;
    (t48445, t48449, t48453, t48458)
}
