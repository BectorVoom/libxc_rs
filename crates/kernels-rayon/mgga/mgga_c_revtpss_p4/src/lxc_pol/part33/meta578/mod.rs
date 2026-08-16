//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta578(t2411: f64, t605: f64, t198: f64, t206: f64, t7086: f64, t25373: f64, t25392: f64, t25386: f64, t25372: f64, t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64, t7059: f64, t9288: f64, t7064: f64, t25305: f64, t136: f64, t2457: f64, t7082: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92790, t92819, t92838, t92843, t92858, t92861) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987(t2411, t605, t198, t206, t7086, t25373, t25392, t25386, t25372, t2435, t25352, t11015, t7018);
        let (t92864, t92870, t92871, t92873, t92875, t92894) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1988(t7048, t822, t25300, t9285, t25299, t7059, t9288, t7064, t25305, t136, t2457, t7082);
    (t92790, t92819, t92838, t92843, t92858, t92861, t92864, t92870, t92871, t92873, t92875, t92894)
}
