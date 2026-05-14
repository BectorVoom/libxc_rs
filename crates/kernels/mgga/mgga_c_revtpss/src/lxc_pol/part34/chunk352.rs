//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 352/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk352<F: Float>(t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t1656: F, t1660: F, t1665: F, t1671: F, t1675: F, t375: F) -> (F,) {
    let t1678 = t1009 + t1011 * t1656 / 288.0 + 0.21437009059034868486e-3 * t1660 * t375 - 0.21437009059034868486e-3 * t1025 * t1665 + 0.21437009059034868486e-3 * t1041 * t1671 + t1060 + 0.14291339372689912324e-3 * t1063 * t1675;
    (t1678,)
}
