//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1117/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1117<F: Float>(t2411: F, t29704: F, t1032: F, t6343: F, t1982: F, t29807: F, t342: F, t19462: F, t1976: F, t7150: F, t19658: F, t7122: F, t19920: F, t25522: F, t27489: F, t4817: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106516 = t29704 * t2411;
    let t106655 = t6343 * t1032;
    let t106656 = t1982 * t106655;
    let t106701 = t342 * t29807;
    let t106727 = t19462 * t1976;
    let t106787 = t7150 * t106655;
    let t106877 = t7122 * t19658;
    let t106896 = t25522 * t19920;
    let t106906 = t27489 * t4817;
    (t106516, t106655, t106656, t106701, t106727, t106787, t106877, t106896, t106906)
}
