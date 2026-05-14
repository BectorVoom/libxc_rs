//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1063/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1063<F: Float>(t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t4911: F) -> (F, F, F, F, F, F, F) {
    let t15745 = t4857 * t1053;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15752 = t11922 * t4901;
    let t15754 = 0.28582678745379824648e-3 * t4899 * t15752;
    let t15769 = t3172 * t4874;
    let t15771 = 0.19055119163586549765e-3 * t3127 * t15769;
    let t15772 = t3172 * t4802;
    let t15774 = 0.3811023832717309953e-3 * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15794 = t11922 * t4911;
    (t15745, t15750, t15754, t15771, t15774, t15776, t15794)
}
