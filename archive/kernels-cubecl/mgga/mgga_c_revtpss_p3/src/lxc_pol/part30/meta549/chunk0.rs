//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1988/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1988<F: Float>(t1882: F, t4056: F, t13867: F, t221: F, t13824: F, t1398: F, t5658: F, t48073: F, t543: F, t3923: F, t48105: F, t14304: F, t4147: F) -> (F, F, F, F, F, F, F) {
    let t48475 = t1882 * t4056;
    let t48525 = t221 * t13867;
    let t48662 = t221 * t13824;
    let t49146 = t5658 * t1398;
    let t49306 = t48073 * t543;
    let t49376 = t49146 * t543;
    let t49380 = t48475 * t543;
    let t49393 = t48105 * t3923;
    let t49564 = t14304 * t4147;
    (t48525, t48662, t49306, t49376, t49380, t49393, t49564)
}
