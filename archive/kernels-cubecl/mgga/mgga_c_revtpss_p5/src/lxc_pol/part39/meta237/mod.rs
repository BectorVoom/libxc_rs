//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta237<F: Float>(t3767: F, t5330: F, t1248: F, t3603: F, t5332: F, t3720: F, t1774: F, t1250: F, t1794: F, t73: F) -> (F, F, F, F, F, F, F, F) {
        let (t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk911::<F>(t3767, t5330, t1248, t3603, t5332, t3720, t1774, t1250, t1794, t73);
    (t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351)
}
