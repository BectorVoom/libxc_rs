//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1365/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1365<F: Float>(t242: F, t240: F, t72: F, t10700: F, t2652: F, t10710: F, t9775: F, t10733: F, t10716: F, t10741: F, t10665: F, t243: F) -> (F, F, F, F, F, F) {
    let t40459 = t242 * t242;
    let t40460 = F::cast_from(1.0_f64) / t40459;
    let t40462 = t240 * t40460 * t72;
    let t40471 = t2652 * t10700;
    let t40473 = t9775 * t10710;
    let t40475 = t9775 * t10733;
    let t40477 = t10716 * t10741;
    let t40479 = t243 * t10665;
    (t40462, t40471, t40473, t40475, t40477, t40479)
}
