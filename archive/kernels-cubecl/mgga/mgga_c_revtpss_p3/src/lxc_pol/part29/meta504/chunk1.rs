//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1822/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1822<F: Float>(t49146: F, t543: F, t48475: F, t3923: F, t48105: F, t14304: F, t4147: F, t1868: F, t4135: F, t116: F, t13424: F, t10871: F, t1558: F) -> (F, F, F, F, F, F, F) {
    let t49376 = t49146 * t543;
    let t49380 = t48475 * t543;
    let t49393 = t48105 * t3923;
    let t49564 = t14304 * t4147;
    let t49582 = t1868 * t4135;
    let t49686 = t13424 * t116;
    let t50474 = t1558 * t10871;
    (t49376, t49380, t49393, t49564, t49582, t49686, t50474)
}
