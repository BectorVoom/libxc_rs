//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 970/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk970<F: Float>(t5985: F, t7025: F, t6019: F, t7038: F, t6030: F, t7045: F, t1558: F, t1579: F, t231: F, t1955: F, t6041: F, t30: F, t6079: F, t1468: F, t1583: F, t6075: F) -> (F, F, F, F, F, F, F, F) {
    let t29629 = t7025 * t5985;
    let t29631 = t7038 * t6019;
    let t29633 = t7045 * t6030;
    let t29682 = t1579 * t1558 * t231;
    let t29698 = t1955 * t6041;
    let t29713 = t30 * t6079;
    let t29716 = t1468 * t1583;
    let t29719 = t30 * t6075;
    (t29629, t29631, t29633, t29682, t29698, t29713, t29716, t29719)
}
