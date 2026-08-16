//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta576(t28814: f64, t689: f64, t94669: f64, t2435: f64, t28902: f64, t7515: f64, t98308: f64, t97962: f64, t14110: f64, t96463: f64, t5775: f64, t7492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t102244, t102246, t102249, t102253, t102255, t102257, t102261) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1902(t28814, t689, t94669, t2435, t28902, t7515, t98308, t97962, t14110, t96463, t5775, t7492);
    (t102244, t102246, t102249, t102253, t102255, t102257, t102261)
}
