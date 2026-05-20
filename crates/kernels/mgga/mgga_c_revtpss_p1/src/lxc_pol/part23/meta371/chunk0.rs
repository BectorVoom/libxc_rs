//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1700/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700<F: Float>(t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t3153: F, t4866: F, t11922: F, t4911: F, t3115: F, t1032: F, t4743: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15769 = t3172 * t4874;
    let t15771 = F::cast_from(0.19055119163586549765e-3_f64) * t3127 * t15769;
    let t15772 = t3172 * t4802;
    let t15774 = F::cast_from(0.3811023832717309953e-3_f64) * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15780 = t4866 * t3153;
    let t15794 = t11922 * t4911;
    let t15796 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t15794;
    let t15816 = t4743 * t1032;
    (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816)
}
