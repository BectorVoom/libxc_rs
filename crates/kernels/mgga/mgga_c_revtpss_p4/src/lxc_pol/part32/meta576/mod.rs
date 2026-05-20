//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta576<F: Float>(t28814: F, t689: F, t94669: F, t2435: F, t28902: F, t7515: F, t98308: F, t97962: F, t14110: F, t96463: F, t5775: F, t7492: F) -> (F, F, F, F, F, F, F) {
        let (t102244, t102246, t102249, t102253, t102255, t102257, t102261) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1902::<F>(t28814, t689, t94669, t2435, t28902, t7515, t98308, t97962, t14110, t96463, t5775, t7492);
    (t102244, t102246, t102249, t102253, t102255, t102257, t102261)
}
