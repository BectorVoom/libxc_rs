//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta399<F: Float>(t12879: F, t1715: F, t247: F, t1261: F, t12916: F, t5342: F, t5340: F, t127: F, t371: F, t5318: F, t1235: F, t3685: F, t5373: F) -> (F, F, F, F, F, F, F) {
        let (t17416, t17417, t17423, t17425, t17435, t17437, t17444) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1449::<F>(t12879, t1715, t247, t1261, t12916, t5342, t5340, t127, t371, t5318, t1235, t3685, t5373);
    (t17416, t17417, t17423, t17425, t17435, t17437, t17444)
}
