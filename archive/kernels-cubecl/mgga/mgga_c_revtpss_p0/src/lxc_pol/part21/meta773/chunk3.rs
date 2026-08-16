//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2748/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748<F: Float>(t1558: F, t2645: F, t10868: F, t2482: F, t814: F, t14547: F, t14671: F, t14686: F, t2661: F, t2662: F, t2754: F, t4416: F) -> (F, F, F) {
    let t50560 = t1558 * t2645;
    let t50570 = t2482 * t10868 * t814;
    let t50573 = t50570 * t14686 * t14671 * t14547;
    let t50577 = t2661 * t2662 * t4416 * t2754;
    (t50560, t50573, t50577)
}
