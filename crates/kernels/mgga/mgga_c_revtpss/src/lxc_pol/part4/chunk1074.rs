//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1074/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1074<F: Float>(t12854: F, t5330: F, t11772: F, t3623: F, t3717: F, t1263: F, t675: F, t1122: F, t247: F, t1261: F, t126: F, t3617: F) -> (F, F, F, F, F, F) {
    let t12855 = t12854 * t5330;
    let t12865 = t3623 * t11772;
    let t12866 = t3717 * t12865;
    let t12879 = t675 * t1263;
    let t12881 = t247 * t12879 * t1122;
    let t12882 = t1261 * t12881;
    let t12884 = t126 * t3617;
    (t12855, t12865, t12866, t12879, t12882, t12884)
}
