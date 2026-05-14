//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 953/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk953<F: Float>(t7507: F, t786: F, t1364: F, t2097: F, t3923: F, t543: F, t7301: F, t25937: F, t7282: F, t10073: F, t1426: F, t2098: F, t3917: F, t25899: F, t26231: F, t72: F, t7531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26252 = t786 * t7507;
    let t26253 = t26252 * t1364;
    let t26255 = t2097 * t3923;
    let t26257 = t7301 * t26255 * t543;
    let t26260 = t25937 * t2097;
    let t26261 = t7282 * t26260;
    let t26263 = 0.24093411633903331839e-3 * t10073 * t26261;
    let t26264 = t2098 * t1426;
    let t26265 = t786 * t26264;
    let t26266 = t26265 * t3917;
    let t26268 = t25899 * t26231;
    let t26270 = t7531 * t72;
    (t26252, t26253, t26255, t26257, t26260, t26261, t26263, t26264, t26265, t26266, t26268, t26270)
}
