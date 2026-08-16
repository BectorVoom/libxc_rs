//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1149/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1149<F: Float>(t3172: F, t4802: F, t1063: F, t4807: F, t11922: F, t4911: F, t3115: F, t1032: F, t4743: F, t1040: F, t11921: F, t247: F, t4757: F) -> (F, F, F, F, F) {
    let t15772 = t3172 * t4802;
    let t15774 = F::cast_from(0.3811023832717309953e-3_f64) * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15794 = t11922 * t4911;
    let t15796 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t15794;
    let t15816 = t4743 * t1032;
    let t15817 = t15816 * t1040;
    let t15827 = t247 * t11921 * t4757;
    (t15774, t15776, t15796, t15817, t15827)
}
