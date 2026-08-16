//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta568<F: Float>(t14767: F, t27159: F, t4537: F, t605: F, t15071: F, t30: F, t25207: F, t61203: F, t4433: F, t892: F, t14749: F, t18875: F, t92790: F) -> (F, F, F, F, F, F, F) {
        let (t98699, t98702, t98705, t98709, t98713, t98716, t98733) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1915::<F>(t14767, t27159, t4537, t605, t15071, t30, t25207, t61203, t4433, t892, t14749, t18875, t92790);
    (t98699, t98702, t98705, t98709, t98713, t98716, t98733)
}
