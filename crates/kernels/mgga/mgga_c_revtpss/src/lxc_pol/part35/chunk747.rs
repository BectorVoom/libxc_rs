//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 747/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk747<F: Float>(t11986: F, t1592: F, t247: F, t1063: F, t11262: F, t1670: F, t1041: F, t1663: F, t371: F, t676: F, t1025: F, t1647: F, t3140: F, t3149: F, t1660: F, t3201: F) -> (F, F, F, F, F, F) {
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15731 = t11262 * t1670;
    let t15732 = t1041 * t15731;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15862 = t1660 * t3201;
    (t15712, t15732, t15750, t15822, t15823, t15862)
}
