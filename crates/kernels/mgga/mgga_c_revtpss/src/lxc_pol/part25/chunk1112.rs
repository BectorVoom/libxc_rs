//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1112/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1112<F: Float>(t25219: F, t7043: F, t820: F, t843: F, t857: F, t2656: F, t7045: F, t240: F, t7036: F, t2664: F, t2661: F, t2670: F, t7033: F) -> (F, F, F, F, F, F, F, F) {
    let t25220 = F::new(0.11337795902333997111e-1) * t25219;
    let t25222 = t820 * t7043 * t843;
    let t25223 = t25222 * t857;
    let t25224 = F::new(0.16006300097412701803e-1) * t25223;
    let t25225 = t7045 * t2656;
    let t25227 = t7036 * t240;
    let t25228 = t25227 * t2664;
    let t25229 = t2661 * t25228;
    let t25230 = F::new(0.28582678745379824648e-4) * t25229;
    let t25231 = t7033 * t2670;
    (t25220, t25222, t25224, t25225, t25227, t25228, t25230, t25231)
}
