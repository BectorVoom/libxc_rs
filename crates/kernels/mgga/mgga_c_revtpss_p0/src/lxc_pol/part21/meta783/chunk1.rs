//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2810/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2810<F: Float>(t2723: F, t2782: F, t4503: F, t51625: F, t10661: F, t14602: F, t1558: F, t2482: F, t10535: F, t14523: F, t9285: F, t10073: F, t14496: F) -> (F, F, F, F) {
    let t51628 = t2782 * t4503 * t51625 * t2723;
    let t51632 = t2482 * t10661 * t1558 * t14602;
    let t51635 = t10535 * t14523 * t9285;
    let t51637 = t10073 * t14496;
    (t51628, t51632, t51635, t51637)
}
